#![allow(unused)]
use std::collections::HashMap;
use crate::packrat::PackRat;

#[derive(Clone)]
struct Attributes<'i> {
	id: Option<&'i str>,
	classes: Vec<&'i str>,
	attributes: HashMap<&'i str, &'i str>
}

#[derive(Clone)]
enum Inline<'i> {
	// Inline Limited
	Strong(&'i str),
	Emphasis(&'i str),
	Mark(&'i str),
	Insert(&'i str),
	Delete(&'i str),
	SuperScript(&'i str),
	SubScript(&'i str),
	Code(&'i str),
	Cite(&'i str),
	Span(&'i str, Attributes<'i>),
	Link(&'i str, Attributes<'i>, &'i str),
	// The rest of Inline
	Image(&'i str, Attributes<'i>, &'i str),
	Tag(&'i str),
	Newline
}
#[derive(Clone)]
enum Block<'i> {
	Paragraph(Vec<Inline<'i>>),
	HorizontalRule(Attributes<'i>),
	Heading(u8, Attributes<'i>, Inline<'i>),
	Code(&'i str, &'i str, Attributes<'i>),
	OrderedList(Attributes<'i>, Vec<Block<'i>>),
	UnorderedList(Attributes<'i>, Vec<Block<'i>>),
	Tag(&'i str, Attributes<'i>, Vec<Block<'i>>),
	Passthrough(&'i str)
}

fn ctori(c: char) -> bool {
	c.is_ascii_alphanumeric() || c == '-' || c == '_'
}
fn key_value<'i>(input: &mut PackRat<'i>) -> Option<(&'i str, &'i str)> {
	let k = input.epat(ctori)?;
	input.epat(':')?;
	let _ = input.epat(char::is_whitespace);
	input.epat('"')?;
	let v = input.epat(|c| c != '"')?;
	input.epat('"')?;
	Some((k, v))
}
fn parse_attributes<'i>(input: &mut PackRat<'i>) -> Option<Attributes<'i>> {
	let mut classes = vec![];
	let mut id = None;
	let mut attributes = HashMap::new();
	loop {
		if let Some(_) = input.epat('.') {
			classes.push(input.epat(ctori)?);
		} else if let Some(_) = input.epat('#') {
			id = Some(input.epat(ctori)?);
		} else if let Some(_) = input.epat('{') {
			let (k, v) = input.epar(key_value)?;
			attributes.insert(k, v);
			while let Some(_) = input.epat(',') {
				let _ = input.epat(char::is_whitespace);
				let (k, v) = input.epar(key_value)?;
				attributes.insert(k, v);
			}
			input.epat('}');
		} else {
			break Some(Attributes {
				classes,
				id,
				attributes,
			});
		}
	}
}

fn parse_strong<'i>(input: &mut PackRat<'i>) -> Option<Inline<'i>> {
	input.epat("*")?;
	let inner = vec![input.epar(parse_inline_limited)?];
	input.epat("*")?;
	Some(Inline::Strong(""))
}

fn parse_inline_limited<'i>(input: &mut PackRat<'i>) -> Option<Inline<'i>> {
	parse_strong(input)
}