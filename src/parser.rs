#![allow(unused)]
use std::collections::HashMap;
use super::parser2::{Input, ParseResult};

#[derive(Debug)]
pub struct Author<'i> {
	pub name: &'i str,
	pub email: Option<&'i str>
}
#[derive(Debug)]
pub struct DocumentHeader<'i> {
	pub title: &'i str,
	pub description: &'i str,
	pub keywords: Vec<&'i str>,
	pub authors: Vec<Author<'i>>,
	pub draft: bool,
	pub meta: HashMap<&'i str, &'i str>
}

fn parse_header_title<'i>(input: &mut Input<'i>) -> ParseResult<&'i str> {
	input.expect_pattern("= ")?;
	input.expect_line()
}
fn parse_header_attribute<'i>(input: &mut Input<'i>) -> ParseResult<(&'i str, &'i str)> {
	input.expect_pattern(':')?;
	let k = input.expect_pattern(|c| c != ':' && c != '\r' && c != '\n')?;
	input.expect_pattern(':')?;
	let v = input.expect_line().or_else(|_| {
		input.expect_lineend()?;
		Ok("")
	})?;
	Ok((k, v.trim()))
}
fn parse_header_keywords<'i>(input: &mut Input<'i>) -> ParseResult<Vec<&'i str>> {
	let keyword = |c: char| c.is_alphabetic() || (c.is_whitespace() && c != '\r' && c != '\n');

	let mut keywords = vec![input.expect_pattern(keyword)?];
	while let Ok(_) = input.expect_pattern(", ") {
		keywords.push(input.expect_pattern(keyword)?);
	}
	input.expect_lineend()?;
	Ok(keywords)
}
fn parse_header_authors<'i>(input: &mut Input<'i>) -> ParseResult<Vec<Author<'i>>> {
	let mut author = |input: &mut Input<'i>| -> ParseResult<Author<'i>> {
		let name = input.expect_pattern(|c: char| c.is_alphabetic() || (c.is_whitespace() && c != '\n'))?;
		let email = if let Ok(_) = input.expect_pattern('<') {
			let email = input.expect_pattern(|c: char| c != '>' && c != '\n')?;
			input.expect_pattern('>')?;
			Some(email)
		} else { None };
		Ok(Author { name, email })
	};
	let mut authors = vec![input.expect(&mut author)?];
	while let Ok(_) = input.expect_pattern("; ") {
		authors.push(input.expect(&mut author)?);
	}
	Ok(authors)
}
pub fn parse_header<'i>(input: &mut Input<'i>) -> ParseResult<DocumentHeader<'i>> {
	let title = input.expect(&mut parse_header_title)?;
	let mut description = "";
	let mut keywords = vec![];
	let mut authors = vec![];
	let mut draft = false;
	let mut meta = HashMap::new();

	// Consume until the first blank line.
	while let Err(_) = input.expect_lineend() {
		if let Ok((k, v)) = parse_header_attribute(input) {
			match k {
				"description" => description = v,
				"keywords" => keywords = parse_header_keywords(&mut Input::from(v)).unwrap(),
				"authors" => authors = parse_header_authors(&mut Input::from(v)).unwrap(),
				"draft" => draft = v.parse::<bool>().unwrap_or(true),
				_ => { meta.insert(k, v); }
			}
		} else {
			match [title.is_empty(), description.is_empty(), keywords.is_empty(), authors.is_empty()] {
				[false, true, _, _] => description = input.expect_line()?,
				[false, false, true, _] => {
					if let Ok(k) = parse_header_keywords(input) {
						keywords = k
					} else {
						return Err(input.error("Header keywords line."));
					}
				},
				[false, false, false, true] => {
					if let Ok(a) = parse_header_authors(input) {
						authors = a
					} else {
						return Err(input.error("Header authors line."));
					}
				},
				_ => return Err(input.error("A property header line, "))
			}
		}
	}

	Ok(DocumentHeader {
		title, description, keywords, authors, draft, meta
	})
}

#[derive(Debug)]
pub struct Document<'i> {
	pub header: DocumentHeader<'i>,
	pub blocks: Vec<Block<'i>>
}

#[derive(Debug)]
pub enum Block<'i> {
	Paragraph(Attributes<'i>, Vec<Inline<'i>>),
	Heading(u8, Attributes<'i>, Vec<Inline<'i>>),
	CodeBlock(&'i str, Attributes<'i>, &'i str),
	HorizontalRule(Attributes<'i>),
	HtmlTag(&'i str, Attributes<'i>, Vec<Block<'i>>),
	UList(Attributes<'i>, Vec<Block<'i>>),
	OList(Attributes<'i>, Vec<Block<'i>>),
	Raw(&'i str)
}

#[derive(Debug)]
pub struct Attributes<'i> {
	classes: Vec<&'i str>,
	id: Option<&'i str>,
	attributes: HashMap<&'i str, &'i str>
}

fn ctori(c: char) -> bool {
	c.is_ascii_alphanumeric() || c == '-' || c == '_'
}
fn parse_attributes<'i>(input: &mut Input<'i>) -> ParseResult<Attributes<'i>> {
	let mut classes = vec![];
	let mut id = None;
	let mut attributes = HashMap::new();
	let mut kv = |input: &mut Input<'i>| -> ParseResult<(&'i str, &'i str)> {
		let k = input.expect_pattern(ctori)?;
		input.expect_pattern(':')?;
		let _ = input.expect_pattern(char::is_whitespace);
		input.expect_pattern('"')?;
		let v = input.expect_pattern(|c| c != '"')?;
		input.expect_pattern('"')?;
		Ok((k, v))
	};
	loop {
		if let Ok(_) = input.expect_pattern('.') {
			classes.push(input.expect_pattern(ctori)?);
		} else if let Ok(_) = input.expect_pattern('#') {
			id = Some(input.expect_pattern(ctori)?);
		} else if let Ok(_) = input.expect_pattern('{') {
			let (k, v) = input.expect(&mut kv)?;
			attributes.insert(k, v);
			while let Ok(_) = input.expect_pattern(',') {
				let _ = input.expect_pattern(char::is_whitespace);
				let (k, v) = input.expect(&mut kv)?;
				attributes.insert(k, v);
			}
			input.expect_pattern('}');
		} else {
			break Ok(Attributes{ classes, id, attributes });
		}
	}
}

fn parse_block<'i>(current_indent: usize, input: &mut Input<'i>) -> ParseResult<Block<'i>> {
	// Try to parse the current indent
	input.expect_pattern(&"\t".repeat(current_indent))?;

	// TODO: Lists and Paragraph

	// Try to parse a block
	if let Ok(_) = input.expect_pattern("---") {
		let attributes = parse_attributes(input)?;
		input.expect_lineend()?;
		Ok(Block::HorizontalRule(attributes))
	} else if let Ok(tag_name) = input.expect_pattern(&mut ctori) {
		let attributes = parse_attributes(input)?;
		input.expect_lineend()?;
		let blocks = input.expect_star(&mut |input: &mut Input<'i>| parse_block(current_indent + 1, input));
		Ok(Block::HtmlTag(tag_name, attributes, blocks))
	} else if let Ok(_) = input.expect_pattern("```") {
		let language = input.expect_pattern(&mut ctori)?;
		let attributes = parse_attributes(input)?;
		input.expect_lineend()?;
		let code = input.expect_antipattern("```")?;
		input.expect_pattern("```")?;
		Ok(Block::CodeBlock(language, attributes, code))
	} else if let Ok(eqs) = input.expect_pattern(|c: char| c == '=') {
		let attributes = parse_attributes(input)?;
		let _ = input.expect_pattern(char::is_whitespace);
		let hc = eqs.len() as u8;
		if hc > 6 {
			return Err(input.error("<Maximum heading is 6>"));
		}
		let title = input.expect_star(&mut parse_inline);
		Ok(Block::Heading(hc, attributes, title))
	} else if let Ok(_) = input.expect_pattern('#') {
		let attributes = parse_attributes(input)?;
		let _ = input.expect_pattern(' ');
		let mut blocks = vec![parse_block(current_indent, input)?];
		while let Ok(b) = parse_block(current_indent + 1, input) {
			
		}
		Ok(Block::OList(attributes, blocks))
	} else if let Ok(_) = input.expect_pattern('*') {
		Ok(Block::UList(parse_attributes(input)?, input.expect_star(&mut |input: &mut Input<'i>| parse_block(current_indent + 1, input))))
	} else {
		// Paragraph
		let attributes = parse_attributes(input)?;
		let _ = input.expect_pattern(char::is_whitespace);
		Ok(Block::Paragraph(attributes, input.expect_star(&mut parse_inline)))
	}
}

#[derive(Debug)]
pub enum Inline<'i> {
	Text(&'i str),
	Span(Vec<Inline<'i>>, Attributes<'i>),
	Image(Vec<Inline<'i>>, Attributes<'i>, &'i str),
	Strong(Vec<Inline<'i>>),
	Emphasis(Vec<Inline<'i>>),
	InlineCode(Vec<Inline<'i>>),
	LineBreak,
	Inserted(Vec<Inline<'i>>),
	Deleted(Vec<Inline<'i>>),
	Marked(Vec<Inline<'i>>),
	Cite(Vec<Inline<'i>>),
	Superscript(Vec<Inline<'i>>),
	Subscript(Vec<Inline<'i>>),
	RawHTML(&'i str),
	Symbol(char)
}

fn parse_inline<'i>(input: &mut Input<'i>) -> ParseResult<Inline<'i>> {
	unimplemented!()
}

pub fn parse_document<'i>(input: &mut Input<'i>) -> ParseResult<Document<'i>> {
	let header = parse_header(input)?;
	let blocks = input.expect_star(&mut |input: &mut Input<'i>| parse_block(0, input));
	input.expect_eoi()?;
	Ok(Document { header, blocks })
}