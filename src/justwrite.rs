use std::collections::HashMap;
use std::str::pattern::Pattern;

#[derive(Default)]
struct InlineFormatting {
	pub strong: bool,
	pub emphasis: bool,
	pub mark: bool,
	pub code: bool,
	pub insert: bool,
	pub delete: bool,
	pub superscript: bool,
	pub subscript: bool,
	pub cite: bool,
}

pub struct Attributes<'i> {
	id: Option<&'i str>,
	classes: Vec<&'i str>,
	attributes: HashMap<&'i str, &'i str>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Kind<'i> {
	// Inline
	Strong,
	Emphasis,
	Mark,
	Code,
	Insert,
	Delete,
	Superscript,
	Subscript,
	Cite,
	Quote,
	Span,
	// Block
	Paragraph,
	Heading(u8),
	OrderedList,
	UnorderedList,
	BlockQuote,
	CodeBlock(&'i str),
	Tag(&'i str),
}
use Kind::*;

pub enum Token<'i> {
	// Text
	Text(&'i str),
	Symbol(char),
	Passthrough(&'i str),
	// Open + Close Blocks / Inlines
	Start(Option<Attributes<'i>>, Kind<'i>),
	End(Kind<'i>),
	// Void tags
	HorizontalRule(Attributes<'i>),
	Image(Attributes<'i>, &'i str, &'i str),
}

pub struct Parser<'i> {
	input: &'i str,
	consumed: usize,
	formatting: InlineFormatting,
	stack: Vec<Kind<'i>>,
	indentation: Vec<&'i str>,
	at_newline: bool,
}
impl<'i> Parser<'i> {
	fn new(input: &'i str) -> Self {
		Self {
			input,
			consumed: 0,
			formatting: InlineFormatting::default(),
			stack: Vec::new(),
			indentation: Vec::new(),
			at_newline: true,
		}
	}
	fn input(&self) -> &'i str {
		&self.input[self.consumed..]
	}
	fn take_until<P: Pattern<'i>>(&mut self, pattern: P) -> &'i str {
		if let Some(i) = self.input().find(pattern) {
			let ret = &self.input()[..i];
			self.consumed += i;
			ret
		} else {
			let ret = self.input();
			self.consumed += ret.len();
			ret
		}
	}
	fn first_char(&mut self) -> Option<char> {
		self.input().chars().nth(0)
	}
	fn top(&self) -> Option<Kind<'i>> {
		self.stack.last().cloned()
	}
}
impl<'i> Iterator for Parser<'i> {
	type Item = Token<'i>;
	fn next(&mut self) -> Option<Self::Item> {
		// TODO: Handle blocks and indentation.
		let input = self.input();
		if input.starts_with('*') {
			if input.
		} else if input.starts_with(')
	}
}
