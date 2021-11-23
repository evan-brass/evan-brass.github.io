use std::path::{Path, PathBuf};
use std::fmt::Write;

#[macro_use] extern crate rocket;

use rocket::fs::NamedFile;
use rocket_sync_db_pools::{database, rusqlite::{self, Connection}};

mod post;
use post::Post;

#[database("sqlite_blog")]
struct BlogDB (pub Connection);

#[get("/")]
async fn index(db: BlogDB) -> Option<String> {
	let posts = db.run(|conn| -> rusqlite::Result<Vec<Post>> {
		let mut stmt = conn.prepare(r"SELECT * FROM posts
			LEFT JOIN posts_tags ON post_id = posts_id
			LEFT JOIN tags on tags_id = tag_id;")?;
		let posts = Post::distinct_from_rows(&mut stmt.query([])?)?;
		Ok(posts)
	}).await.ok()?;
	let mut ret = String::new();
	let r = &mut ret;

	for post in posts {
		ret.write_fmt(format_args!("{}", post.title)).ok()?;
	}

	Some(ret)
}

#[get("/blog/<slug>")]
async fn single_post(slug: String, db: BlogDB) -> Option<String> {
	let mut posts = db.run(move |conn| -> rusqlite::Result<Vec<Post>> {
		let mut stmt = conn.prepare(r"SELECT * FROM posts WHERE post_slug = ?1
			LEFT JOIN posts_tags on post_id = posts_id
			LEFT JOIN tags on tags_id = tag_id;")?;
		let posts = Post::distinct_from_rows(&mut stmt.query([slug])?)?;
		Ok(posts)
	}).await.ok()?;
	let post = posts.pop()?;
	assert_eq!(posts.len(), 0);
	let ret = format!("");

	Some(ret)
}

#[get("/<file..>")]
async fn static_files(file: PathBuf) -> Option<NamedFile> {
	NamedFile::open(Path::new("static/").join(file)).await.ok()
}

#[launch]
fn rocket() -> _ {
	rocket::build()
		.mount("/", routes![index, static_files, single_post])
		.attach(BlogDB::fairing())
}
