use std::str::Lines;
use std::iter::Peekable;

use crate::packrat::{LexRes, Lexer as LexerTrait};

const CONTROLS: &'static str = "*_`+-^~#\"'@![]()=";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Token<'i> {
	Indent, Dedent,
	Newline, BlankLine,
	Control(char),
	Tag<&'i str>
}
enum LexerState {
	Newline,
	Indent(usize),
	Str
}
pub struct Lexer<'i> {
	state: LexerState,
	lines: Peekable<Lines<'i>>,
	indentation: Vec<&'i str>
}
impl<'i> Iterator for Lexer<'i> {
	type Item = LexRes<'i, Token, ()>;
	fn next(&mut self) -> Option<Self::Item> {
		if let Some(l) = self.lines.peek() {
			match self.state {
				LexerState::Newline => {
					self.state = LexerState::Indent(0);
					return Some(LexRes::Token(if l.trim_start().is_empty() {
						self.lines.next();
						while self.lines.peek().map(|s| s.trim_start().is_empty()).unwrap_or(false) {
							self.lines.next();
						}
						Token::BlankLine
					} else {
						Token::Newline
					}))
				},
				LexerState::Indent(i) => {
					
					self.lines.peek_mut()
				},
				LexerState::Str => {
	
				}
			}
		} else if self.indentation.len() > 0 {
			self.indentation.pop();
			Some(LexRes::Token(Token::Dedent))
		} else {
			None
		}
		if let Some(l) = self.lines.peek() {
			// Send a newline / blankline if needed
			if !self.sent_newline {
				self.sent_newline = true;
				
			}
			// Match against our current indentation
			let mut line = *l;
			for indent in self.indentation.iter() {
				if let Some(rest) = line.strip_prefix(*indent) {
					line = rest;
				} else {
					self.indentation.pop();
					return Some(LexRes::Token(Token::Dedent));
				}
			}
			// Check if there's any more indentation
			let no_indent = line.trim_start();
			if no_indent.len() < line.len() {
				self.indentation.push(&line[..(line.len() - no_indent.len())]);
				return Some(LexRes::Token(Token::Indent));
			}
			// Return the rest of the line
			self.sent_newline = false;
			self.lines.next();
			// TODO: Handle symbols.
			return Some(LexRes::Text(no_indent))
		}
	}
}
impl<'i> LexerTrait<'i> for Lexer<'i> {
	type Token = Token;
	type LexError = ();
	fn new(s: &'i str) -> Self {
		Self {
			lines: s.lines().peekable(),
			indentation: Vec::new(),
			state: LexState::Indent(0)
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use Token::{Indent, Dedent, Newline, BlankLine};

	#[test]
	fn t1() {
		let input = r"This is 
a multi line string.
	with some indents,
	  using mixed ws,
	but it should all,
             
		   
			work out";
		assert_eq!(
			Lexer::new(input).collect::<Vec<_>>(),
			vec![
				LexRes::Text("This is "),
				LexRes::Token(Newline),
				LexRes::Text("a multi line string."),
				LexRes::Token(Newline),
				LexRes::Token(Indent),
				LexRes::Text("with some indents,"),
				LexRes::Token(Newline),
				LexRes::Token(Indent),
				LexRes::Text("using mixed ws,"),
				LexRes::Token(Newline),
				LexRes::Token(Dedent),
				LexRes::Text("but it should all,"),
				LexRes::Token(BlankLine),
				LexRes::Token(Indent),
				LexRes::Text("work out"),
				LexRes::Token(Dedent),
				LexRes::Token(Dedent),
			]
		);
	}
}