#![feature(option_result_contains)]
use std::path::{PathBuf, Path};
use std::fs::File;
use std::error::Error;
use std::sync::mpsc::channel;
use std::time::Duration;
use std::collections::HashMap;
use std::collections::VecDeque;

use chrono::NaiveDate;
use rusqlite::{Connection, Params};
use rusqlite::types::FromSql;
use tera::{Tera, Context, Value};
use voca_rs::strip::strip_tags;
use voca_rs::manipulate::slugify;
use rusqlite::functions::FunctionFlags;

use notify::{Watcher, RecursiveMode, watcher, DebouncedEvent};

mod post;
mod schema;
// use post::Post;

fn get_file<B: AsRef<Path>, T: AsRef<Path>>(base: B, tail: T) -> Result<File, Box<dyn Error>> {
	let mut dest_path = Path::new("public").join(base);
	dest_path.push(tail);
	match dest_path.extension() {
		Some(_) => {
			std::fs::create_dir_all(dest_path.parent().unwrap())?;
		},
		None => {
			std::fs::create_dir_all(&dest_path)?;
			dest_path.push("index.html");
		}
	}

	Ok(File::create(dest_path)?)
}

fn copy_dir_contents(from: &Path, to: &Path) -> Result<(), Box<dyn Error>> {
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
			} else if ft.is_file() {
				std::fs::copy(
					&path,
					dest
				)?;
			}
		}
	}

	Ok(())
}

fn attach_query<P: Params>(
	conn: &mut Connection,
	sql: &str,
	params: P,
	ctx: &mut Context,
	key: &str
) -> Result<(), Box<dyn Error>> {
	let tx = conn.transaction()?;
	let mut stmt = tx.prepare_cached(sql)?;
	let columns_a = stmt.columns();
	let columns: Vec<(String, Option<String>)> = columns_a.iter().map(|c| 
		(c.name().to_owned(), c.decl_type().map(|s| s.to_owned()))
	).collect();
	let mut rows = stmt.query(params)?;
	let mut ret = Vec::new();
	while let Some(row) = rows.next()? {
		let mut v: HashMap<String, Value> = HashMap::new();
		for (i, (key, decl_type)) in columns.iter().enumerate() {
			let vref = row.get_ref(i)?;
			v.insert(key.clone(), if decl_type.contains(&"json") {
				serde_json::from_str(vref.as_str()?)?
			} else {
				Value::column_result(vref)?
			});
		}
		ret.push(v);
	}
	drop(rows);
	stmt.discard();
	tx.finish()?;
	ctx.insert(key, &ret);
	Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
	let db_loc = Path::new("blog.sqlite");
	let templates_loc = Path::new("templates");

	let (tx, rx) = channel();
	let mut watcher = watcher(tx, Duration::from_secs(5))?;

	let mut conn = Connection::open(db_loc)?;
	conn.create_scalar_function(
		"strip_tags",
		1,
		FunctionFlags::SQLITE_DETERMINISTIC | FunctionFlags::SQLITE_UTF8,
		|context| -> rusqlite::Result<String> {
			Ok(strip_tags(context.get_raw(0).as_str()?))
		}
	)?;
	conn.create_scalar_function(
		"slugify",
		1,
		FunctionFlags::SQLITE_DETERMINISTIC | FunctionFlags::SQLITE_UTF8,
		|context| -> rusqlite::Result<String> {
			Ok(slugify(context.get_raw(0).as_str()?))
		}
	)?;
	schema::schema(&conn)?;
	watcher.watch(db_loc, RecursiveMode::NonRecursive)?;
	
	let mut tera = Tera::new("templates/**/*.html")?;
	watcher.watch(templates_loc, RecursiveMode::Recursive)?;
	tera.register_filter("fmtdate", |v: &Value, args: &HashMap<String, Value>| {
		let s = v.as_str().ok_or("fmtdate can only be called on string values.")?;
		let fmt = args.get("fmt").ok_or("must provide a fmt argument which is the format string.")?.as_str().ok_or("fmt argument must be a string")?;
		let d = NaiveDate::parse_from_str(s, "%F").map_err(|e|
			tera::Error::chain("Failed to parse datetime", e)
		)?;
		Ok(d.format(fmt).to_string().into())
	});
	tera.register_filter("includestatic", |v: &Value, _args: &HashMap<String, Value>| {
		let p = v.as_str().ok_or("value must be a string")?;
		let s = std::fs::read_to_string(Path::new("static").join(p))?;
		Ok(s.into())
	});

	loop {
		// Copy the static files over.
		copy_dir_contents(Path::new("static"), Path::new("public"))?;

		// Build the site:
		let mut context = Context::new();
		attach_query(&mut conn, r"
			SELECT title, description, slugify(strip_tags(posts.title)) as slug FROM posts
			ORDER BY posts.published
		;", [], &mut context, "posts")?;
		tera.render_to(
			"post_index.html",
			&context,
			get_file("blog", "index.html")?
		)?;

		// Output the individual post pages
		// for post in posts.iter() {
		// 	let mut context = Context::new();
		// 	context.insert("post", post);
		// 	tera.render_to(
		// 		"single.html",
		// 		&context,
		// 		get_file("blog", &post.slug)?
		// 	)?;
		// }

		// Output the blog index
		// let mut context = Context::new();
		// context.insert("posts", &posts);
		// tera.render_to(
		// 	"post_index.html",
		// 	&context,
		// 	get_file("blog", "index.html")?
		// )?;

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