use std::collections::HashMap;
use std::iter::Peekable;
use std::str::pattern::Pattern;

mod lexer;
use crate::packrat::{LexRes, Lexer as LexerTrait};
use lexer::{Lexer, Token};

pub fn take_prefix<'i, P: Pattern<'i>>(input: &mut &'i str, p: P) -> Option<&'i str> {
	let rest = input.strip_prefix(p)?;
	let prefix = &input[..(input.len() - rest.len())];
	*input = rest;
	Some(prefix)
}
pub fn take_until<'i, P: Pattern<'i>>(input: &mut &'i str, p: P) -> &'i str {
	if let Some(i) = input.find(p) {
		let ret = &input[..i];
		*input = &input[i..];
		ret
	} else {
		let ret = *input;
		*input = &input[input.len()..];
		ret
	}
}

#[derive(Debug, Clone, PartialEq)]
struct Attributes<'i> {
	id: Option<&'i str>,
	classes: Vec<&'i str>,
	attributes: HashMap<&'i str, &'i str>
}
impl<'i> Attributes<'i> {
	fn new(input: &mut &'i str) -> Self {
		let mut id = None;
		if take_prefix(input, "#").is_some() {
			id = Some(take_until(input, |c: char| !c.is_ascii_alphanumeric()));
		}
		let mut classes = Vec::new();
		while take_prefix(input, ".").is_some() {
			classes.push(take_until(input, |c: char| !c.is_ascii_alphanumeric()));
		}
		let mut attributes = HashMap::new();
		if take_prefix(input, "{").is_some() {
			unimplemented!()
		}
		Self { id, classes, attributes }
	}
}

#[rustfmt::skip]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Kind<'i> {
	// Inline:
	Strong, Emphasis, Mark,
	Code, Quote, Cite,
	Insert, Delete,
	Super, Sub,
	Span, Link,
	// Blocks
	Heading(u8),
	Paragraph,
	Tag(&'i str),
	CodeBlock(&'i str),
	BlockQuote,
	OrderedList, UnorderedList,
}
#[derive(Debug, Clone, PartialEq)]
enum AST<'i> {
	Text(&'i str),
	// Open / Close
	Start(Option<Attributes<'i>>, Kind<'i>),
	End(Kind<'i>),
	// Void tags
	Rule(Attributes<'i>),
	Image(Attributes<'i>, &'i str, &'i str),
	LineBreak,
}

struct Parser<'i> {
	beginning: bool,
	lexer: Peekable<Lexer<'i>>,
	stack: Vec<Kind<'i>>
}
impl<'i> Parser<'i> {
	fn is_empty(&mut self) -> bool {
		self.lexer.peek() == Some(&LexRes::Text(""))
	}
	pub fn new(input: &'i str) -> Self {
		Self {
			beginning: true,
			lexer: Lexer::new(input).peekable(),
			stack: Vec::new()
		}
	}
}
impl<'i> Iterator for Parser<'i> {
	type Item = AST<'i>;
	fn next(&mut self) -> Option<Self::Item> {
		if self.is_empty() {
			self.beginning = true;
			self.lexer.next();
		}
		match self.lexer.peek_mut() {
			Some(LexRes::Text(t)) => {
				if self.beginning {
					self.beginning = false;
					// Block elements
					if take_prefix(t, "---").is_some() {
						let attributes = Attributes::new(t);
						assert!(self.is_empty());
						self.lexer.next();
						let temp = self.lexer.peek();
						assert!(
							temp.is_none() ||
							temp == Some(&LexRes::Token(Token::Newline)) ||
							temp == Some(&LexRes::Token(Token::BlankLine))
						);
						Some(AST::Rule(attributes))
					} else {
						unimplemented!()
					}
				} else {
					// Inline elements
					unimplemented!()
				}
			},
			Some(LexRes::Token(Token::Dedent)) => {
				// pop a block off the stack?
				if let Some(top) = self.stack.pop() {
					match top {
						Kind::Heading(_) | Kind::Paragraph |
						Kind::Tag(_) | Kind::CodeBlock(_) |
						Kind::BlockQuote | Kind::OrderedList |
						Kind::UnorderedList => self.next(),
						_ => panic!("Unexpected Dedent Token. {:?}", top)
					}
				} else {
					panic!("Unexpected Dedent Token: Stack was empty.")
				}
			},
			Some(LexRes::Token(_)) => {
				// Handle other stuff?
				unimplemented!()
			},
			_ => None
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn t1() {
		let mut t = "#this.cant.be.real";
		assert_eq!(Attributes::new(&mut t), Attributes {
			id: Some("this"),
			classes: vec!["cant", "be", "real"],
			attributes: HashMap::new()
		});
		assert!(t.is_empty());
	}
}