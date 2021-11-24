use std::path::{Path, PathBuf};
use std::fmt::Display;
use templating::template;

#[macro_use] extern crate rocket;

use rocket::fs::NamedFile;
use rocket::response::content::Html;
use rocket_sync_db_pools::{database, rusqlite::{self, Connection}};

mod post;
use post::Post;

#[database("sqlite_blog")]
struct BlogDB (pub Connection);

const HEADER_STYLES: &'static str = include_str!("../static/header-styles.css");

fn header<'a>(title: &'a str, description: &'a str) -> impl Display + 'a {
	template!(
r#"<!DOCTYPE html>
	<html lang="en">
	<head>
			<meta charset="utf-8">
			<meta name="viewport" content="width=device-width, initial-scale=1">
			<title>"# {title} r#"</title>
			<meta name="description" content=""# {description} r#"">
			<style>"# {HEADER_STYLES} r#"</style>
			<link rel="preload" as="style" href="/css/main.css" onload="this.onload=null;this.rel='stylesheet'">
			<link rel="stylesheet" media="print" href="/css/print.css">
	</head>
	<body>
		<header>
			<h1>Evan Brass</h1>
			<nav>
				<a href=""# {uri!(index())} r#"">Home</a>
				<a href="/about/">About</a>
				<a href="/blog/">Blog</a>
			</nav>
		</header>
		<main>"#)
}

fn footer() -> impl Display {
	template!(
r#"		</main>
	<footer>
		<a href="https://twitter.com/evan_brass">Twitter</a> and <a href="https://github.com/evan-brass">GitHub</a>
	</footer>
	<noscript>
		<link rel="stylesheet" href="/css/main.css">
	</noscript>
</body>
</html>"#)
}

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
	Html(template!(
		{header("Evan Brass", "His life and times.")}
r"			<ol>"
			[posts.iter().map(|p| template!(
				r#"<li>
					<h2><a href=""# {uri!(single_post(&p.slug))} r#"">"# {p.title} r"</a></h2>
					<p>" {p.description} r"</p>
				</li>"))]
r#"			</ol>"#
		{footer()}
	).to_string())
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
		Some(Html(template!(
			{header(&post.title, &post.description)}
r#"			<article itemscope itemtype="https://schema.org/BlogPosting">
				<header>
					<!-- TODO: Social Media image
					<img itemprop="image" src="" alt="">
					-->
					<h1 itemprop="headline">"# {post.title} r#"</h1>
					<span itemprop="author">Evan Brass</span>"#
					(move |fmt| {
						if let Some(published) = post.published {
							Display::fmt(&template!(
r#"					<time itemprop="datePublished" datetime=""# {published.format("%F")} r#"">"#
						{published.format("%B %-d, %Y")}
					"</time>"
							), fmt)
						} else {
							Ok(())
						}
					})
r#"					<ul itemprop="keywords">"#
					[post.tags.iter().map(|t| template!("<li>" {t.name} "</li>"))]
				r#"</ul>
					<nav is="blog-contents">
					<!-- TODO -->
					</nav>
				</header>
				<div itemprop="articleBody">"#
					{post.content}
r#"				</div>
			</article>"#
			{footer()}
		).to_string()))
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
}
