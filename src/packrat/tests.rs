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
	let input = r"[][{}]{}{{}{}}";
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
	A, B, C
}
struct ABCLex<'i> {
	input: &'i str,
	consumed: usize
}
impl<'i> Iterator for ABCLex<'i> {
	type Item = (&'i str, Result<Option<ABCToken>, ()>);
	fn next(&mut self) -> Option<Self::Item> {
		if self.consumed == self.input.len() {
			None
		} else {
			let input = &self.input[self.consumed..];
			if let Some(i) = input.find(|c: char| c == 'a' || c == 'b' || c == 'c') {
				self.consumed += i + 1;
				Some((&input[..i], Ok(Some(match &input[i..(i+1)] {
					"a" => ABCToken::A,
					"b" => ABCToken::B,
					"c" => ABCToken::C,
					_ => unreachable!()
				}))))
			} else {
				self.consumed = self.input.len();
				Some((input, Ok(None)))
			}
		}
	}
}
impl<'i> Lexer<'i> for ABCLex<'i> {
	type Specials = ABCToken;
	type LexError = ();
	fn new(input: &'i str) -> Self {
		Self {input, consumed: 0}
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