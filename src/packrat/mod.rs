#![allow(unused)]
use std::any::{type_name, Any, TypeId};
use std::collections::HashMap;
use std::panic::Location;
use std::str::pattern::Pattern;
use std::iter::Fuse;

pub fn take_pattern<'i, P: Pattern<'i>>(input: &mut &'i str, p: P) -> Option<&'i str> {
	let rest = input.strip_prefix(p)?;
	let mut prefix = &input[..(input.len() - rest.len())];
	*input = rest;
	Some(prefix)
}

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, PartialEq)]
pub enum LexRes<'i, S, E> {
	// Some lexers emit only text or only token items.
	Text(&'i str),
	Token(S),
	LexError(E),
}
pub trait Lexer<'i>: Iterator<Item = LexRes<'i, Self::Token, Self::LexError>> {
	type Token: 'i + PartialEq + Clone + Copy;
	type LexError: 'i + std::fmt::Debug + Clone;

	fn new(s: &'i str) -> Self;
}

pub struct NoLex<'i>(&'i str);
impl<'i> Iterator for NoLex<'i> {
	type Item = LexRes<'i, (), ()>;
	fn next(&mut self) -> Option<Self::Item> {
		if !self.0.is_empty() {
			let ret = LexRes::Text(self.0);
			self.0 = "";
			Some(ret)
		} else {
			None
		}
	}
}
impl<'i> Lexer<'i> for NoLex<'i> {
	type Token = ();
	type LexError = ();
	fn new(s: &'i str) -> Self {
		Self(s)
	}
}

pub type Parser<'i, L, O> = fn(&mut PackRat<'i, L>) -> Option<O>;

#[derive(Debug)]
pub enum ParseError<'i, L: Lexer<'i>> {
	LexError {
		caller: &'static Location<'static>,
		e: L::LexError,
	},
	ExpectedToken {
		caller: &'static Location<'static>,
		expected: L::Token,
	},
	ExpectedPattern {
		caller: &'static Location<'static>,
		pattern: String, // TODO: How to represent the pattern?
	},
}

#[derive(Debug)]
pub struct PackRat<'i, L: Lexer<'i> = NoLex<'i>> {
	lexer: Fuse<L>,
	tokens: Vec<LexRes<'i, L::Token, L::LexError>>,
	tokens_index: usize,
	str_consumed: usize,
	errors: Vec<(usize, usize, ParseError<'i, L>)>,
	// TODO: add a map that stores rescursive depth.
	memo: HashMap<(usize, usize, usize), Option<(usize, usize, Box<()>)>>,
}
impl<'i, L: Lexer<'i>> PackRat<'i, L> {
	pub fn new(s: &'i str) -> Self {
		Self {
			lexer: L::new(s).fuse(),
			tokens: Vec::new(),
			tokens_index: 0,
			str_consumed: 0,
			errors: Vec::new(),
			memo: HashMap::new(),
		}
	}
	fn get_lex_res(&mut self) -> Option<LexRes<'i, L::Token, L::LexError>> {
		if let Some(LexRes::Text(t)) = self.tokens.get(self.tokens_index) {
			if t.len() == self.str_consumed {
				self.tokens_index += 1;
				self.str_consumed = 0;
			}
		}
		if self.tokens_index == self.tokens.len() {
			let tok = self.lexer.next()?;
			self.tokens.push(tok);
			self.str_consumed = 0; // I'm not certain about this... but it's working...
		}
		Some(self.tokens[self.tokens_index].clone())
	}
	// Get the next token but don't advance the token index.
	// #[track_caller]
	pub fn get_token(&mut self) -> Option<L::Token> {
		if let LexRes::Token(tok) = self.get_lex_res()? {
			Some(tok)
		} else {
			None
		}
	}
	// Get the next input but don't advance the consumed or token index.
	// #[track_caller]
	pub fn get_input(&mut self) -> Option<&'i str> {
		if let LexRes::Text(text) = self.get_lex_res()? {
			Some(&text[self.str_consumed..])
		} else {
			None
		}
	}
	// #[track_caller]
	pub fn etok<T: PartialEq<L::Token>>(&mut self, tok: T) -> Option<L::Token> {
		let t = self.get_token()?;
		if tok == t {
			self.tokens_index += 1;
			Some(t)
		} else {
			None
		}
	}
	// #[track_caller]
	pub fn epat<P: Pattern<'i>>(&mut self, pat: P) -> Option<&'i str> {
		let i = self.get_input()?;
		if let Some(postfix) = i.strip_prefix(pat) {
			let prefix_len = i.len() - postfix.len();
			let prefix = &i[..prefix_len];
			self.str_consumed += prefix_len;
			Some(prefix)
		} else {
			self.errors.push((
				self.tokens_index,
				self.str_consumed,
				ParseError::ExpectedPattern {
					caller: Location::caller(),
					pattern: type_name::<P>().into(),
				},
			));
			None
		}
	}
	// #[track_caller]
	pub fn epar<O: Clone>(&mut self, par: Parser<'i, L, O>) -> Option<O> {
		if let Some(ret) = self
			.memo
			.get(&(self.tokens_index, self.str_consumed, par as usize))
		{
			if let Some((new_i, new_c, o)) = ret {
				self.tokens_index = *new_i;
				self.str_consumed = *new_c;
				let o = unsafe { std::mem::transmute::<&Box<()>, &Box<O>>(o) };
				Some(*o.clone())
			} else {
				None
			}
		} else {
			let old_str_consumed = self.str_consumed;
			let old_tokens_index = self.tokens_index;
			let ret = par(self);
			self.memo.insert(
				(old_tokens_index, old_str_consumed, par as usize),
				ret.clone().map(|o| {
					(self.tokens_index, self.str_consumed, unsafe {
						std::mem::transmute::<Box<O>, Box<()>>(Box::new(o.clone()))
					})
				}),
			);
			if ret.is_none() {
				// Restore packrat state if the parser failed to parse.
				self.str_consumed = old_str_consumed;
				self.tokens_index = old_tokens_index;
			}
			ret
		}
	}
	// #[track_caller]
	pub fn is_eoi(&mut self) -> bool {
		self.tokens_index == self.tokens.len() && self.get_lex_res().is_none()
	}
	// TODO: Support Left Recursion.
}
