use std::path::{Path, PathBuf};

#[macro_use] extern crate rocket;

use rocket::fs::NamedFile;
use rocket::response::content::Html;
use rocket_sync_db_pools::{database, rusqlite::{self, Connection}};
use rocket_dyn_templates::Template;

mod post;
use post::Post;
mod template;
use template::{post_single, post_list};

#[database("sqlite_blog")]
struct BlogDB (pub Connection);

#[get("/")]
async fn index(db: BlogDB) -> Html<String> {
	let posts = db.run(|conn| -> rusqlite::Result<Vec<Post>> {
		let mut stmt = conn.prepare(r"SELECT * FROM posts
			LEFT JOIN posts_tags ON post_id = posts_id
			LEFT JOIN tags on tags_id = tag_id
			ORDER BY post_published
			;")?;
		let posts = Post::distinct_from_rows(&mut stmt.query([])?)?;
		Ok(posts)
	}).await.unwrap();
	let mut ret = String::new();
	post_list(&mut ret, &posts).unwrap();

	Html(ret)
}

#[get("/blog/<slug>")]
async fn single_post(slug: String, db: BlogDB) -> Option<Html<String>> {
	let mut posts = db.run(move |conn| -> rusqlite::Result<Vec<Post>> {
		let mut stmt = conn.prepare(r"SELECT * FROM posts
		LEFT JOIN posts_tags ON post_id = posts_id
		LEFT JOIN tags on tags_id = tag_id
		WHERE post_slug = ?1
		;").unwrap();
		let posts = Post::distinct_from_rows(&mut stmt.query([slug])?)?;
		Ok(posts)
	}).await.unwrap();
	if let Some(post) = posts.pop() {
		let mut ret = String::new();
		post_single(&mut ret, &post).unwrap();
		Some(Html(ret))
	} else {
		None
	}
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
		.attach(Template::fairing())
}
