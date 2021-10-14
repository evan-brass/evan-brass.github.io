use std::collections::HashMap;

pub trait Parser<O> {
	fn parse<'i>(&self, input: &'i str) -> Option<(O, &'i str)>;
}

impl<O, T> Parser<O> for T where T: for<'i> Fn(&'i str) -> Option<(O, &'i str)> {
	fn parse<'i>(&self, input: &'i str) -> Option<(O, &'i str)> {
		self(input)
	}
}

pub struct ElementAttributes<'i> {
	classes: Vec<&'i str>,
	id: Option<&'i str>,
	attributes: HashMap<&'i str, &'i str>
}
pub enum Block<'i> {
	Element {
		tag: &'i str,
		attributes: ElementAttributes<'i>
	},
	Code {
		language: &'i str,
		content: &'i str
	},
	HorizontalRule,
	OrderedList {
		attributes: ElementAttributes<'i>,

	}
}
pub enum Inline<'i> {
	Text (&'i str),
	Symbol (&'i str),
	Inserted (Vec<Inline<'i>>),
	Deleted (Vec<Inline<'i>>),
	Superscript (Vec<Inline<'i>>),
	Subscript (Vec<Inline<'i>>),
	Link (Vec<Inline<'i>>, &'i str),
	Image (Vec<Inline<'i>>, &'i str), // TODO: Float / alignment
	Code (Vec<Inline<'i>>),
	Emphasis (Vec<Inline<'i>>),
	Strong (Vec<Inline<'i>>)
}

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

fn parse_header_title(line: &str) -> Option<&str> {
	line.strip_prefix("= ")
}
fn parse_header_attribute(line: &str) -> Option<(&str, &str)> {
	let (k, v) = line.strip_prefix(":")?.split_once(":")?;
	Some((k, v.trim()))
}
fn parse_header_keywords(line: &str) -> Option<Vec<&str>> {
	let mut keywords = vec![];
	for keyword in line.split(", ") {
		if keyword.contains(|c: char| !c.is_alphabetic() && !c.is_whitespace()) {
			return None
		}
		keywords.push(keyword);
	}
	Some(keywords)
}
fn parse_header_authors<'i>(line: &'i str) -> Option<Vec<Author<'i>>> {
	let mut authors = vec![];
	for author in line.split("; ") {
		authors.push(if let Some((name, email)) = author.split_once("<") {
			Author {
				name: name.trim(),
				email: Some(email.trim().strip_suffix(">")?)
			}
		} else {
			Author {
				name: author.trim(),
				email: None
			}
		});
	}
	Some(authors)
}
pub fn parse_header<'i>(input: &'i str) -> (DocumentHeader<'i>, &str) {
	let mut title = "";
	let mut description = "";
	let mut keywords = vec![];
	let mut authors = vec![];
	let mut draft = false;
	let mut meta = HashMap::new();
	let mut rest = input;

	while let Some((line, r)) = non_blank(rest) {
		if let Some(t) = parse_header_title(line) {
			title = t;
		} else if let Some((k, v)) = parse_header_attribute(line) {
			match k {
				"title" => title = v,
				"description" => description = v,
				"keywords" => keywords = parse_header_keywords(v).unwrap(),
				"authors" => authors = parse_header_authors(v).unwrap(),
				"draft" => draft = v.parse::<bool>().unwrap_or(true),
				_ => { meta.insert(k, v); }
			}
		} else {
			match [title.is_empty(), description.is_empty(), keywords.is_empty(), authors.is_empty()] {
				[false, true, _, _] => description = line,
				[false, false, true, _] if let Some(k) = parse_header_keywords(line) => {
					keywords = k
				},
				[false, false, false, true] if let Some(a) = parse_header_authors(line) => {
					authors = a
				},
				_ => println!("skipped line in header: {}", line)
			}
		}
		rest = r;
	}

	(DocumentHeader {
		title, description, keywords, authors, draft, meta
	}, rest)
}

fn non_blank(input: &str) -> Option<(&str, &str)> {
	line(input).and_then(|(s, r)| if s.is_empty() {
		None
	} else {
		Some((s, r))
	})
}

fn line(input: &str) -> Option<(&str, &str)> {
	if input.is_empty() {
		None
	} else {
		input.split_once("\n").or(Some((input, "")))
	}
}