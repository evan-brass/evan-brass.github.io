use super::Lexer;

#[derive(Debug, PartialEq)]
pub enum BasicLexError {
	PastEnd
}
impl std::fmt::Display for BasicLexError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Debug::fmt(self, f)
	}
}
impl std::error::Error for BasicLexError{}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BasicTokenKind {
	Text,
	End
}
pub struct BasicLexer<'i> {
	input: &'i str,
	consumed: usize
}
impl<'i> Lexer<'i> for BasicLexer<'i> {
	type TokenKind = BasicTokenKind;
	type LexError = BasicLexError;
	fn get_token(&mut self) -> Result<(Self::TokenKind, &'i str), Self::LexError> {
		if self.consumed > self.input.len() {
			return Err(BasicLexError::PastEnd);
		}
		let input = &self.input[self.consumed..];
		if self.consumed == self.input.len() {
			self.consumed += 1;
			Ok((BasicTokenKind::End, &self.input[self.input.len()..]))
		} else {
			let l = input.chars().nth(0).unwrap().len_utf8();
			self.consumed += l;
			Ok((BasicTokenKind::Text, &input[..l]))
		}
	}
}
impl<'i> From<&'i str> for BasicLexer<'i> {
	fn from(input: &'i str) -> Self {
		Self { input, consumed: 0 }
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn t1() {
		let mut blex = BasicLexer::from("Test ❄");
		assert_eq!(blex.get_token().unwrap(), (BasicTokenKind::Text, "T"));
		assert_eq!(blex.get_token().unwrap(), (BasicTokenKind::Text, "e"));
		assert_eq!(blex.get_token().unwrap(), (BasicTokenKind::Text, "s"));
		assert_eq!(blex.get_token().unwrap(), (BasicTokenKind::Text, "t"));
		assert_eq!(blex.get_token().unwrap(), (BasicTokenKind::Text, " "));
		assert_eq!(blex.get_token().unwrap(), (BasicTokenKind::Text, "❄"));
		assert_eq!(blex.get_token().unwrap(), (BasicTokenKind::End, ""));
		assert_eq!(blex.get_token(), Err(BasicLexError::PastEnd));
	}
}