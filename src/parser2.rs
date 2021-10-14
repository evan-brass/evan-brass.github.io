use std::panic::Location;
use std::str::pattern::Pattern;
use std::any::type_name;
use std::fmt::Display;
use std::fmt::Write;
use std::str::pattern::SearchStep;
use std::str::pattern::Searcher;

#[derive(Debug)]
pub struct ParseError {
	caller: &'static Location<'static>,
	expected: &'static str,
	at: (usize, usize),
	line: String,
}
impl Display for ParseError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		writeln!(f, "Parsing failed, {} expected {} at col {}:", self.caller, self.expected, self.at.1)?;
		writeln!(f, r#"{}: "{}""#, self.at.0, self.line)
	}
}
type ParseResult<O> = Result<O, ParseError>;

#[derive(Debug)]
pub struct Input<'i> {
	lines: Vec<usize>,
	input: &'i str,
	consumed: usize
}
impl<'i> From<&'i str> for Input<'i> {
	fn from(s: &'i str) -> Self {
		Self {
			lines: s.match_indices('\n').map(|(i, _)| i).collect(),
			input: s,
			consumed: 0
		}
	}
}
impl<'i> Input<'i> {
	fn line_idx(&self, index: usize) -> usize {
		let t = self.lines.binary_search(&index);
		println!("{:?}", t);
		match t {
			Ok(n) => n,
			Err(n) => n
		}
	}
	fn ln_cn_line(&self, index: usize) -> (usize, usize, &str) {
		let li = self.line_idx(self.consumed);
		let line_start = self.lines.get(li).cloned().unwrap_or(0);
		let cn = self.consumed - line_start;
		let line_end = self.lines.get(li + 1).cloned().unwrap_or(self.input.len());
		(li + 1, cn, &self.input[line_start..line_end])
	}
	#[track_caller]
	fn error(&self, expected: &'static str) -> ParseError {
		let (ln, cn, line) = self.ln_cn_line(self.consumed);
		ParseError {
			caller: Location::caller(),
			expected,
			at: (ln, cn),
			line: line.into()
		}
	}
	#[track_caller]
	pub fn expect<P: Parser<'i>>(&mut self, p: &mut P) -> ParseResult<P::Output> {
		let last_consumed = self.consumed;
		match p.parse(self) {
			Ok(o) => Ok(o),
			Err(e) => {
				self.consumed = last_consumed;
				Err(e) // TODO: Make a backtrace instead of just passing the bottom error
			}
		}
	}
	#[track_caller]
	pub fn expect_star<P: Parser<'i>>(&mut self, p: &mut P) -> Vec<P::Output> {
		let mut v = vec![];
		while let Ok(o) = self.expect(p) {
			v.push(o);
		}
		v
	}
	#[track_caller]
	pub fn expect_pattern<P: Pattern<'i>>(&mut self, p: P) -> ParseResult<&'i str> {
		let mut searcher = p.into_searcher(&self.input[self.consumed..]);
		let mut b = 0;
		while let SearchStep::Match(_, nb) = searcher.next() {
			b = nb;
		}
		if b > 0 {
			let ret = Ok(&self.input[self.consumed..self.consumed + b]);
			self.consumed += b;
			ret
		} else {
			Err(self.error(type_name::<P>()))
		}
	}
}

pub trait Parser<'i> {
	type Output: 'i;
	fn parse(&mut self, input: &mut Input<'i>) -> ParseResult<Self::Output>;
}
impl<'i, O: 'i, T: FnMut(&mut Input<'i>) -> ParseResult<O>> Parser<'i> for T {
	type Output = O;
	fn parse(&mut self, input: &mut Input<'i>) -> ParseResult<Self::Output> {
		self(input)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_line_col() {
		let input = Input::from("\nHello World!\nThis is a test of some stuff.");
		println!("{:?}", input);
		assert_eq!(input.line_idx(5), 1);
		assert_eq!(input.line_idx(0), 0);
		assert_eq!(input.line_idx(13), 1);
		assert_eq!(input.line_idx(14), 2);
	}
}