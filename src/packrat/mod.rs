#![allow(unused)]
use std::any::{type_name, Any, TypeId};
use std::collections::HashMap;
use std::panic::Location;
use std::str::pattern::Pattern;

#[cfg(test)]
mod tests;

pub trait Lexer<'i>:
	Iterator<Item = (&'i str, Result<Option<Self::Specials>, Self::LexError>)>
{
	type Specials: 'i + PartialEq + Clone + Copy;
	type LexError: 'i + std::fmt::Debug + Clone;

	fn new(s: &'i str) -> Self;
}

pub struct NoLex<'i>(&'i str);
impl<'i> Iterator for NoLex<'i> {
	type Item = (&'i str, Result<Option<()>, ()>);
	fn next(&mut self) -> Option<Self::Item> {
		if !self.0.is_empty() {
			let ret = Some((self.0, Ok(None)));
			self.0 = "";
			ret
		} else {
			None
		}
	}
}
impl<'i> Lexer<'i> for NoLex<'i> {
	type Specials = ();
	type LexError = ();
	fn new(s: &'i str) -> Self {
		Self(s)
	}
}

pub trait Parser<'i, L: Lexer<'i>>: Any {
	type Output: 'i + Clone;
	fn parse(&self, pr: &mut PackRat<'i, L>) -> Option<Self::Output>;
	fn is_recursive(&self) -> bool { false } // Potential problem... We have a blanket implementation of Parser for functions which doesn't handle them being potentially recursive.
}
impl<'i, O, L, T> Parser<'i, L> for T
where
	O: 'i + Clone,
	L: Lexer<'i>,
	T: 'static + Fn(&mut PackRat<'i, L>) -> Option<O>,
{
	type Output = O;
	fn parse(&self, pr: &mut PackRat<'i, L>) -> Option<Self::Output> {
		self(pr)
	}
}

#[derive(Debug)]
pub enum ParseError<'i, L: Lexer<'i>> {
	LexError {
		caller: &'static Location<'static>,
		e: L::LexError,
	},
	ExpectedToken {
		caller: &'static Location<'static>,
		expected: L::Specials,
	},
	ExpectedPattern {
		caller: &'static Location<'static>,
		pattern: String, // TODO: How to represent the pattern?
	},
}

#[derive(Debug)]
pub struct PackRat<'i, L: Lexer<'i> = NoLex<'i>, C = Box<dyn Any>> {
	lexer: L,
	lexer_done: bool,
	tokens: Vec<(&'i str, Result<Option<L::Specials>, L::LexError>)>,
	tokens_index: usize,
	str_consumed: usize,
	errors: Vec<(usize, usize, ParseError<'i, L>)>,
	// TODO: add a map that stores rescursive depth.
	memo: HashMap<(usize, usize, TypeId), C>, // TODO: Memoize the rules and handle left recursion
}
impl<'i, L: Lexer<'i>> PackRat<'i, L> {
	pub fn new(s: &'i str) -> Self {
		Self {
			lexer: L::new(s),
			lexer_done: false,
			tokens: Vec::new(),
			tokens_index: 0,
			str_consumed: 0,
			errors: Vec::new(),
			memo: HashMap::new(),
		}
	}
	fn get_lex_res(&mut self) -> Option<(&'i str, Result<Option<L::Specials>, L::LexError>)> {
		if self.tokens_index == self.tokens.len() {
			if !self.lexer_done {
				if let Some(tok) = self.lexer.next() {
					// The token must either have a lex error or a special or there must be some input:
					// assert!(tok.1.is_err() || tok.1.unwrap_or(None).is_some() || !tok.0.is_empty());
					self.tokens.push(tok);
					self.str_consumed = 0; // I'm not certain about this...
				} else {
					self.lexer_done = true;
					return None;
				}
			} else {
				return None;
			}
		}
		Some(self.tokens[self.tokens_index].clone())
	}
	// Get the next token but don't advance the token index.
	// #[track_caller]
	pub fn get_token(&mut self) -> Option<L::Specials> {
		let tok = self.get_lex_res()?;
		if self.str_consumed == tok.0.len() {
			match tok.1 {
				Ok(None) => {
					self.tokens_index += 1;
					self.get_token()
				}
				Ok(Some(s)) => Some(s),
				Err(e) => {
					self.errors.push((
						self.tokens_index,
						self.str_consumed,
						ParseError::LexError {
							caller: Location::caller(),
							e,
						},
					));
					None
				}
			}
		} else {
			None
		}
	}
	// Get the next input but don't advance the consumed or token index.
	// #[track_caller]
	pub fn get_input(&mut self) -> Option<&'i str> {
		let tok = self.get_lex_res()?;
		if self.str_consumed == tok.0.len() {
			if let Ok(None) = tok.1 {
				self.str_consumed = 0;
				self.tokens_index += 1;
				self.get_input()
			} else {
				None
			}
		} else {
			Some(&tok.0[self.str_consumed..])
		}
	}
	// TODO: Enable retreiving a single pattern / token without expecting it?
	// #[track_caller]
	pub fn etok<T: PartialEq<L::Specials>>(&mut self, tok: T) -> Option<L::Specials> {
		let t = self.get_token()?;
		if tok == t {
			self.tokens_index += 1;
			Some(t)
		} else {
			None
		}
	}
	// #[track_caller]
	pub fn epat<P: Pattern<'i> + Any>(&mut self, pat: P) -> Option<&'i str> {
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
	pub fn epar<P: Parser<'i, L>>(&mut self, par: P) -> Option<P::Output> {
		let old_str_consumed = self.str_consumed;
		let old_tokens_index = self.tokens_index;
		let ret = par.parse(self);
		if ret.is_none() {
			// Restore packrat state if the parser failed to parse.
			self.str_consumed = old_str_consumed;
			self.tokens_index = old_tokens_index;
		}
		ret
	}
	// #[track_caller]
	pub fn is_eoi(&self) -> bool {
		self.lexer_done && self.tokens_index == self.tokens.len()
	}
	// TODO: Support Left Recursion.
}
