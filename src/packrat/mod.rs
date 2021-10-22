use std::panic::Location;
mod blex;

pub trait Lexer<'i> {
	type TokenKind: std::fmt::Debug + PartialEq + Clone + Copy;
	type LexError: 'i + std::error::Error;
	fn get_token(&mut self) -> Result<(Self::TokenKind, &'i str), Self::LexError>;
}
type Parser<O, L> = fn(&mut PackRat<L>) -> Option<O>;
type Token<'i, L: Lexer<'i>> = (L::TokenKind, &'i str);

pub struct ParseError <'i, L: Lexer<'i>> {
	caller: &'static Location<'static>,
	expected: Option<Token<'i, L>>,
	found: Token<'i, L>
}

pub struct PackRat<'i, L: Lexer<'i>> {
	lexer: L,
	tokens: Vec<(L::TokenKind, &'i str)>,
	index: usize,
	saved: Vec<usize>,
	lex_error: Option<L::LexError>,
	errors: Vec<(usize, ParseError<'i, L>)>,
	// TODO: Memoize the rules and handle left recursion
}
impl<'i, L: Lexer<'i>> PackRat<'i, L> {
	fn token(&mut self) -> Option<Token<'i, L>> {
		if self.lex_error.is_some() {
			return None;
		}
		assert!(self.index <= self.tokens.len());
		if let Some(t) = self.tokens.get(self.index).cloned() {
			Some(t)
		} else {
			match self.lexer.get_token() {
				Ok(n) => {
					self.tokens.push(n);
					Some(n)
				},
				Err(e) => {
					// TODO: Save the lex Error and report it?
					self.lex_error = Some(e);
					None
				}
			}
		}
	}
	// Expect a Token
	#[track_caller]
	pub fn etok<T: PartialEq<Token<'i, L>>>(&mut self, t: T) -> Option<&'i str> {
		let tok = self.token()?;
		if t.eq(&tok) {
			self.index += 1;
			Some(tok.1)
		} else {
			
			self.errors.push((self.index, ParseError {
				caller: Location::caller(),
				expected,
				found: tok
			}));
			None
		}
	}
	// Expect a Parser
	#[track_caller]
	pub fn epar<O>(&mut self, p: Parser<O, L>) -> Option<O> {
		self.saved.push(self.index);
		if let Some(o) = p(self) {
			self.saved.pop().unwrap();
			Some(o)
		} else {
			self.index = self.saved.pop().unwrap();
			None
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn t1() {

	}
}