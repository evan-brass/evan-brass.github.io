#![feature(option_result_contains, hash_set_entry)]

use std::collections::HashMap;
use std::collections::VecDeque;
use std::error::Error;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::sync::mpsc::channel;
use std::time::Duration;
use std::cell::RefCell;
use std::sync::Mutex;
use std::collections::HashSet;

use chrono::NaiveDate;
use rusqlite::CachedStatement;
use rusqlite::functions::FunctionFlags;
use rusqlite::types::ValueRef;
use rusqlite::{Connection, Params};
use rusqlite::Row;
use tera::{Context, Tera, Value};
use voca_rs::manipulate::slugify;
use voca_rs::strip::strip_tags;
use serde::{Serialize, Serializer, ser::Error as _, ser::SerializeSeq as _, ser::SerializeStruct};

use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};

mod schema;

fn get_file<B: AsRef<Path>, T: AsRef<Path>>(base: B, tail: T) -> Result<File, Box<dyn Error>> {
	let mut dest_path = Path::new("public").join(base);
	dest_path.push(tail);
	match dest_path.extension() {
		Some(_) => {
			std::fs::create_dir_all(dest_path.parent().unwrap())?;
		}
		None => {
			std::fs::create_dir_all(&dest_path)?;
			dest_path.push("index.html");
		}
	}

	Ok(File::create(dest_path)?)
}

fn create_links_recursive(from: &Path, to: &Path) -> Result<(), Box<dyn Error>> {
	let mut directories = VecDeque::new();
	directories.push_back(PathBuf::from(from));
	while let Some(p) = directories.pop_front() {
		for entry in std::fs::read_dir(p)? {
			let entry = entry?;
			let ft = entry.file_type()?;
			let path = entry.path();
			let dest = to.join(path.strip_prefix(from).unwrap());

			if ft.is_dir() {
				std::fs::create_dir_all(dest)?;
				directories.push_back(path);
			} else if ft.is_file() && !dest.exists() {
				std::fs::hard_link(path, dest)?;
			}
		}
	}

	Ok(())
}
lazy_static::lazy_static!{
	pub static ref COLUMN_NAMES: Mutex<HashSet<&'static str>> = Mutex::new(HashSet::new());
}
enum ColumnType {
	// TODO: Dates and shit
	None,
	Json,
	Other(String)
}
impl From<Option<&str>> for ColumnType {
	fn from(s: Option<&str>) -> Self {
		match s {
			None => ColumnType::None,
			Some("json") => ColumnType::Json,
			Some(t) => ColumnType::Other(t.to_owned())
		}
	}
}
fn get_columns(stmt: &CachedStatement) -> Vec<(&'static str, ColumnType)> {
	stmt.columns().into_iter().map(|c| {
		(*COLUMN_NAMES.lock().unwrap().get_or_insert_with(c.name(), |n| {
			Box::leak(n.to_owned().into_boxed_str())
		}), From::from(c.decl_type()))
	}).collect()
}

struct RowWrapper<'a> (&'a Vec<(&'static str, ColumnType)>, &'a Row<'a>);
impl<'a> Serialize for RowWrapper<'a> {
	fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		// TODO: Let the struct be named
		// TODO: Date and DateTime types
		let mut struc = serializer.serialize_struct("row", self.0.len())?;
		for (key, decl) in self.0.iter() {
			let vr = self.1.get_ref(*key).map_err(|e| S::Error::custom(e))?;
			match (vr, decl) {
				(ValueRef::Null, _) => struc.skip_field(*key)?,
				(ValueRef::Integer(ref i), _) => struc.serialize_field(*key, i)?,
				(ValueRef::Real(ref r), _) => struc.serialize_field(*key, r)?,
				(ValueRef::Text(s), ColumnType::Json) => struc.serialize_field(*key,
					&serde_json::from_slice::<serde_json::Value>(s).map_err(|e| S::Error::custom(e))?
				)?,
				(ValueRef::Text(s), _) => struc.serialize_field(*key, std::str::from_utf8(s).map_err(
					|e| S::Error::custom(e)
				)?)?,
				(ValueRef::Blob(b), _) => struc.serialize_field(*key, b)?
			}
		}
		struc.end()
	}
}

struct Query<'conn, P: Params + Clone> {
	stmt: RefCell<CachedStatement<'conn>>,
	params: P
}
impl<'conn, P: Params + Clone> Query<'conn, P> {
	pub fn new(stmt: CachedStatement<'conn>, params: P) -> Self {
		Self{ stmt: RefCell::new(stmt), params }
	}
}
impl<'conn, P: Params + Clone> Serialize for Query<'conn, P> {
	fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		let mut stmt = self.stmt.borrow_mut();
		let columns = get_columns(&stmt);
		let mut rows = stmt.query(self.params.clone()).map_err(|e| S::Error::custom(e))?;

		let mut seq = serializer.serialize_seq(None)?;
		while let Some(row) = rows.next().map_err(|e| S::Error::custom(e))? {
			seq.serialize_element(&RowWrapper(&columns, row))?;
		}
		seq.end()
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	let db_loc = Path::new("blog.sqlite");
	let templates_loc = Path::new("templates");

	let (tx, rx) = channel();
	let mut watcher = watcher(tx, Duration::from_secs(5))?;

	let conn = Connection::open(db_loc)?;
	conn.create_scalar_function(
		"strip_tags",
		1,
		FunctionFlags::SQLITE_DETERMINISTIC | FunctionFlags::SQLITE_UTF8,
		|context| -> rusqlite::Result<String> { Ok(strip_tags(context.get_raw(0).as_str()?)) },
	)?;
	conn.create_scalar_function(
		"slugify",
		1,
		FunctionFlags::SQLITE_DETERMINISTIC | FunctionFlags::SQLITE_UTF8,
		|context| -> rusqlite::Result<String> { Ok(slugify(context.get_raw(0).as_str()?)) },
	)?;
	schema::schema(&conn)?;
	watcher.watch(db_loc, RecursiveMode::NonRecursive)?;

	let mut tera = Tera::new("templates/**/*.html")?;
	watcher.watch(templates_loc, RecursiveMode::Recursive)?;
	tera.register_filter("fmtdate", |v: &Value, args: &HashMap<String, Value>| {
		let s = v
			.as_str()
			.ok_or("fmtdate can only be called on string values.")?;
		let fmt = args
			.get("fmt")
			.ok_or("must provide a fmt argument which is the format string.")?
			.as_str()
			.ok_or("fmt argument must be a string")?;
		let d = NaiveDate::parse_from_str(s, "%F")
			.map_err(|e| tera::Error::chain("Failed to parse datetime", e))?;
		Ok(d.format(fmt).to_string().into())
	});
	tera.register_filter(
		"includestatic",
		|v: &Value, _args: &HashMap<String, Value>| {
			let p = v.as_str().ok_or("value must be a string")?;
			let s = std::fs::read_to_string(Path::new("static").join(p))?;
			Ok(s.into())
		},
	);

	// Create links for the static files/folders
	create_links_recursive(Path::new("static"), Path::new("public"))?;

	loop {
		// Build the site:
		let mut context = Context::new();
		let stmt = conn.prepare_cached(
			"SELECT title, description, slug FROM posts \
			ORDER BY posts.published;"
		)?;
		context.insert("posts", &Query::new(stmt, []));
		tera.render_to("post_index.html", &context, get_file("blog", "index.html")?)?;

		// Output the individual post pages
		let mut stmt = conn.prepare_cached(
			"SELECT * FROM posts \
			ORDER BY posts.published;"
		)?;
		let columns = get_columns(&stmt);
		let mut rows = stmt.query([])?;
		while let Some(row) = rows.next()? {
			let mut context = Context::new();
			context.insert("post", &RowWrapper(&columns, row));
			let slug = row.get_ref("slug")?.as_str()?;
			tera.render_to(
				"single.html",
				&context,
				get_file("blog", slug)?
			)?;
		}

		println!("Site built.");
		// Wait until we need to rebuild the site again
		loop {
			let e = rx.recv()?;
			if let DebouncedEvent::Write(p) = e {
				println!("{:?}", p);
				if p.starts_with(templates_loc) {
					println!("Templates changed");
					tera.full_reload()?;
				}
				break;
			}
		}
	}
}
