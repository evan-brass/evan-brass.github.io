use super::*;
#[test]
fn t1() {
	let test = "This is a simple test.";
	let mut pr: PackRat<NoLex> = PackRat::new(test);
	assert_eq!(pr.is_eoi(), false);
	assert_eq!(pr.epat("This"), Some("This"));
	assert_eq!(pr.epat(char::is_whitespace), Some(" "));
	assert_eq!(pr.epat("psych!"), None);
	assert_eq!(pr.epat("is"), Some("is"));
	assert_eq!(pr.epat(" a simple test."), Some(" a simple test."));
	assert_eq!(pr.epat(|_: char| true), None);
}

#[test]
fn t2() {
	#[derive(Debug, PartialEq, Clone)]
	enum AST {
		Bracket(Vec<AST>),
		Brace(Vec<AST>),
	}
	let input = "[][{}]{}{{}{}}";
	fn parse_items(pr: &mut PackRat) -> Vec<AST> {
		let mut items = Vec::new();
		loop {
			if let Some(i) = pr.epar(parse_brackets) {
				items.push(i);
			} else if let Some(i) = pr.epar(parse_braces) {
				items.push(i);
			} else {
				break items;
			}
		}
	}
	fn parse_brackets(pr: &mut PackRat) -> Option<AST> {
		pr.epat("[")?;
		let children = parse_items(pr);
		pr.epat("]")?;
		Some(AST::Bracket(children))
	}
	fn parse_braces(pr: &mut PackRat) -> Option<AST> {
		pr.epat("{")?;
		let children = parse_items(pr);
		pr.epat("}")?;
		Some(AST::Brace(children))
	}

	assert_eq!(
		parse_items(&mut PackRat::new(input)),
		vec![
			AST::Bracket(vec![]),
			AST::Bracket(vec![AST::Brace(vec![])]),
			AST::Brace(vec![]),
			AST::Brace(vec![AST::Brace(vec![]), AST::Brace(vec![])])
		]
	)
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum ABCToken {
	A,
	B,
	C,
}
struct ABCLex<'i> {
	input: &'i str,
}
impl<'i> Iterator for ABCLex<'i> {
	type Item = LexRes<'i, ABCToken, ()>;
	fn next(&mut self) -> Option<Self::Item> {
		if self.input.is_empty() {
			None
		} else {
			let is_abc = |c: char| c == 'a' || c == 'b' || c == 'c';
			if let Some(c) = take_pattern(&mut self.input, is_abc) {
				Some(LexRes::Token(match c {
					"a" => ABCToken::A,
					"b" => ABCToken::B,
					"c" => ABCToken::C,
					_ => unreachable!(),
				}))
			} else {
				let end = self.input.find(is_abc).unwrap_or(self.input.len());
				let ret = &self.input[..end];
				self.input = &self.input[end..];
				Some(LexRes::Text(ret))
			}
		}
	}
}
impl<'i> Lexer<'i> for ABCLex<'i> {
	type Token = ABCToken;
	type LexError = ();
	fn new(input: &'i str) -> Self {
		Self { input }
	}
}

#[test]
fn t3() {
	let input = "this b is a test of a cabweird lexer.";
	let mut pr: PackRat<ABCLex> = PackRat::new(input);
	assert_eq!(pr.epat("this b is a"), None);
	assert_eq!(pr.epat("this"), Some("this"));
	assert_eq!(pr.epat(char::is_whitespace), Some(" "));
	assert_eq!(pr.epat(|_: char| true), None); // Make sure that we can't read any more characters until we handle the b token
	assert_eq!(pr.etok(ABCToken::A), None);
	assert_eq!(pr.etok(ABCToken::B), Some(ABCToken::B));
	assert_eq!(pr.epat(" is "), Some(" is "));
	assert_eq!(pr.etok(ABCToken::A), Some(ABCToken::A));
	assert_eq!(pr.epat(" test of "), Some(" test of "));
	assert_eq!(pr.etok(ABCToken::A), Some(ABCToken::A));
	assert_eq!(pr.epat(char::is_whitespace), Some(" "));
	assert_eq!(pr.etok(ABCToken::C), Some(ABCToken::C));
	assert_eq!(pr.etok(ABCToken::A), Some(ABCToken::A));
	assert_eq!(pr.etok(ABCToken::B), Some(ABCToken::B));
	assert_eq!(pr.epat("weird lexer."), Some("weird lexer."));
}
