#![feature(option_result_contains, hash_set_entry, once_cell)]
#[macro_use]
extern crate rocket;

use std::{collections::HashMap, path::{Path, PathBuf}, io::Cursor};

use rusqlite::params;
use rocket::{fs::NamedFile, Response, http::Header, response::Responder, data::Data};
use rocket_dyn_templates::Template;

mod context;
mod db;
mod helpers;
use self::{context::{Query, RowWrapper}	, db::CONNECTION, helpers::{fmtdate, includestatic}};


#[get("/")]
fn home() -> Template {
	CONNECTION.with(|conn| {
		Template::render(
			"index",
			HashMap::from([(
				"posts",
				Query::new_sql(conn, "SELECT * FROM posts ORDER BY posts.published;", []),
			)]),
		)
	})
}

#[get("/blog")]
fn blog_index() -> Template {
	CONNECTION.with(|conn| {
		Template::render(
			"blog_index",
			HashMap::from([(
				"posts",
				Query::new_sql(conn, "SELECT slug, title, published, keywords, substr(strip_tags(posts.content), 0, 100) AS opening FROM posts ORDER BY posts.published;", [])
			)])
		)
	})
}

struct File {
	data: Vec<u8>,
	content_type: String
}
impl<'r> Responder<'r, 'static> for File {
	fn respond_to(self, _request: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
		Ok(Response::build()
		.header(Header::new("Content-Type", self.content_type))
		.sized_body(self.data.len(), Cursor::new(self.data))
		.finalize())
	}
}

#[get("/files/<file>")]
fn file<'a>(file: String) -> Option<File> {
	let (id, ext) = file.split_once('.')?;
	let id: usize = id.parse().ok()?;
	CONNECTION.with(|conn| {
		conn.query_row(r"SELECT data, type FROM files WHERE id = ?1, extension = ?2;", params![id, ext], |row| {
			let data: Vec<u8> = row.get("data")?;
			let content_type: String = row.get("type")?;
			Ok(File{ data, content_type })
		}).ok()
	})
}

#[post("/files/<file>", data = "<data>")]
fn upload_file(file: String, data: Data) -> Option<()> {

	Some(())
}

#[get("/blog/<slug>")]
fn blog_post(slug: String) -> Option<Template> {
	CONNECTION.with(|conn| {
		let mut stmt = conn.prepare_cached("SELECT * FROM posts WHERE posts.slug == ?1").unwrap();
		stmt.query_row([slug], |row| {
			Ok(Template::render("single", RowWrapper::new(row)))
		}).ok()
	})
}

#[get("/tags")]
fn tag_index() -> Template {
	CONNECTION.with(|conn| {
		Template::render(
			"tag_index",
			HashMap::from([(
				"tags",
				Query::new_sql(conn, "SELECT DISTINCT json_each(posts.keywords) AS tag_name FROM posts;", []),
			)]),
		)
	})
}
// TODO: Individual tag pages
// TODO: Gallery

#[get("/<path..>")]
async fn static_files(path: PathBuf) -> Option<NamedFile> {
	NamedFile::open(Path::new("static").join(path)).await.ok()
}

#[launch]
fn rocket() -> _ {
	rocket::build()
		.mount("/", routes![
			home,
			blog_index,
			blog_post,
			static_files,
			tag_index,
			file,
			upload_file
		])
		.attach(Template::custom(|engines| {
			engines.handlebars.set_strict_mode(true);
			engines
				.handlebars
				.register_helper("fmtdate", Box::new(fmtdate));
			engines
				.handlebars
				.register_helper("includestatic", Box::new(includestatic));
		}))
}
