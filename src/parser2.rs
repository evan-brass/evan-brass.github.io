use std::panic::Location;
use std::str::pattern::Pattern;
use std::any::type_name;
use std::fmt::Display;
use std::str::pattern::SearchStep;
use std::str::pattern::Searcher;
use std::error::Error;

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
impl Error for ParseError {}

pub type ParseResult<O> = Result<O, ParseError>;

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
#[allow(unused)]
impl<'i> Input<'i> {
	fn input(&self) -> &'i str {
		&self.input[self.consumed..]
	}
	fn line_idx(&self, index: usize) -> usize {
		let t = self.lines.binary_search(&index);
		match t {
			Ok(n) => n,
			Err(n) => n
		}
	}
	fn ln_cn_line(&self, index: usize) -> (usize, usize, &str) {
		let li = self.line_idx(index);
		let line_start = if li == 0 {
			0
		} else {
			self.lines[li - 1] + 1
		};
		let cn = index - line_start;
		let line_end = self.lines.get(li).cloned().unwrap_or(self.input.len());
		(li + 1, cn, &self.input[line_start..line_end])
	}
	#[track_caller]
	pub fn error(&self, expected: &'static str) -> ParseError {
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
	// TODO: So... I'm not really sure how patterns are supposed to work and I'm not sure if the searcher is actually supposed to be in a loop or not.  Should input.expect_pattern("aa") take all of the a's in "aaaaaa" or not? Fuck.
	#[track_caller]
	pub fn expect_pattern<P: Pattern<'i>>(&mut self, p: P) -> ParseResult<&'i str> {
		let mut searcher = p.into_searcher(self.input());
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
	#[track_caller]
	pub fn expect_antipattern<P: Pattern<'i>>(&mut self, p: P) -> ParseResult<&'i str> {
		let mut searcher = p.into_searcher(self.input());
		let mut b = 0;
		Ok(loop {
			match searcher.next() {
				SearchStep::Done => {
					break self.input();
				},
				SearchStep::Reject(_, nb) => {
					b = nb;
				},
				_ => {
					let ret = &self.input[self.consumed..self.consumed + b];
					self.consumed += b;
					break ret;
				}
			}
		})
	}
	#[track_caller]
	pub fn expect_lineend(&mut self) -> ParseResult<()> {
		// Expect a line termination or the end of input
		if self.consumed == self.input.len() {
			Ok(())
		} else if self.input().starts_with("\r\n") {
			self.consumed += 2;
			Ok(())
		} else if self.input().starts_with('\n') {
			self.consumed += 1;
			Ok(())
		} else {
			Err(self.error("<Newline or EOI>"))
		}
	}
	#[track_caller]
	pub fn expect_eoi(&mut self) -> ParseResult<()> {
		if self.consumed == self.input.len() {
			Ok(())
		} else {
			Err(self.error("<EOI>"))
		}
	}
	#[track_caller]
	pub fn expect_line(&mut self) -> ParseResult<&'i str> {
		// Take a string until hitting a newline (consuming both the string and the newline)
		let ret = self.expect_pattern(|c: char| c != '\r' && c != '\n')?;
		self.expect_lineend()?;
		Ok(ret)
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
impl<'i> Parser<'i> for &'static str {
	type Output = &'i str;
	fn parse(&mut self, input: &mut Input<'i>) -> ParseResult<Self::Output> {
		input.expect_pattern(*self)
	}
}

// pub struct PatternParser<P>(P);
// impl<'i, P: Pattern<'i>> Parser<'i> for PatternParser<P> {
// 	type Output = &'i str;
// 	fn parse(&mut self, input: &mut Input<'i>) -> ParseResult<Self::Output> {
// 		input.expect_pattern(self.0)
// 	}
// }

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