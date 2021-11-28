use std::path::Path;
use std::fs::File;
use std::error::Error;
use std::sync::mpsc::channel;
use std::time::Duration;

use rusqlite::Connection;
use tera::{Tera, Context};

use notify::{Watcher, RecursiveMode, watcher};

mod post;
use post::Post;

// const HEADER_STYLES: &'static str = include_str!("../static/header-styles.css");

// #[get("/")]
// async fn index(db: BlogDB) -> Html<String> {
// 	let posts = db.run(|conn| -> rusqlite::Result<Vec<Post>> {
// 		let mut stmt = conn.prepare(r"SELECT * FROM posts
// 			LEFT JOIN posts_tags ON post_id = posts_id
// 			LEFT JOIN tags on tags_id = tag_id
// 			ORDER BY post_published
// 			;")?;
// 		let posts = Post::distinct_from_rows(&mut stmt.query([])?)?;
// 		Ok(posts)
// 	}).await.unwrap();
// 	Html(template!(
// 		{header("Evan Brass", "His life and times.")}
// r"			<ol>"
// 			[posts.iter().map(|p| template!(
// 				r#"<li>
// 					<h2><a href=""# {uri!(single_post(&p.slug))} r#"">"# {p.title} r"</a></h2>
// 					<p>" {p.description} r"</p>
// 				</li>"))]
// r#"			</ol>"#
// 		{footer()}
// 	).to_string())
// }

// #[get("/blog/<slug>")]
// async fn single_post(slug: String, db: BlogDB) -> Option<Html<String>> {
// 	let mut posts = db.run(move |conn| -> rusqlite::Result<Vec<Post>> {
// 		let mut stmt = conn.prepare(r"SELECT * FROM posts
// 		LEFT JOIN posts_tags ON post_id = posts_id
// 		LEFT JOIN tags on tags_id = tag_id
// 		WHERE post_slug = ?1
// 		;").unwrap();
// 		let posts = Post::distinct_from_rows(&mut stmt.query([slug])?)?;
// 		Ok(posts)
// 	}).await.unwrap();
// 	if let Some(post) = posts.pop() {
// 		Some(Html(template!(
// 			{header(&post.title, &post.description)}
// r#"			<article itemscope itemtype="https://schema.org/BlogPosting">
// 				<header>
// 					<!-- TODO: Social Media image
// 					<img itemprop="image" src="" alt="">
// 					-->
// 					<h1 itemprop="headline">"# {post.title} r#"</h1>
// 					<span itemprop="author">Evan Brass</span>"#
// 					(move |fmt| {
// 						if let Some(published) = post.published {
// 							Display::fmt(&template!(
// r#"					<time itemprop="datePublished" datetime=""# {published.format("%F")} r#"">"#
// 						{published.format("%B %-d, %Y")}
// 					"</time>"
// 							), fmt)
// 						} else {
// 							Ok(())
// 						}
// 					})
// r#"					<ul itemprop="keywords">"#
// 					[post.tags.iter().map(|t| template!("<li>" {t.name} "</li>"))]
// 				r#"</ul>
// 					<nav is="blog-contents">
// 					<!-- TODO -->
// 					</nav>
// 				</header>
// 				<div itemprop="articleBody">"#
// 					{post.content}
// r#"				</div>
// 			</article>"#
// 			{footer()}
// 		).to_string()))
// 	} else {
// 		None
// 	}
// }

// #[get("/<file..>")]
// async fn static_files(file: PathBuf) -> Option<NamedFile> {
// 	NamedFile::open(Path::new("static/").join(file)).await.ok()
// }

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

// fn build_site(conn: &mut Connection) -> Result<(), Box<dyn Error>> {
// 	// Link static files
// 	// Output 404
// 	// Output home index page
// 	// Output tags index page
// 	// Output tag pages
// 	// Output rss
// 	// Output gallery
// 	let mut stmt = conn.prepare(r"
// 		SELECT * FROM posts
// 		LEFT JOIN posts_tags on post_id = posts_id
// 		LEFT JOIN tags on tags_id = tag_id
// 		ORDER BY post_published
// 	;")?;

// 	let posts = Post::distinct_from_rows(&mut stmt.query([])?)?;
// 	let posts = &posts;

// 	// Output blog index page
// 	let mut destination = get_file("blog", "index.html")?;
// 	write!(&mut destination, "{}", template!(
// 		{header("Blog | Evan Brass", "His life and times.")}
// 		r"<ol>"
// 			[posts.iter().map(|p| template!(
// 				r#"<li>
// 					<h2><a href="/blog/"# {p.slug} r#"">"# {p.title} r"</a></h2>
// 					<p>" {p.description} r"</p>
// 				</li>"))]
// 		r#"</ol>"#
// 		{footer()}
// 	))?;

// 	// Output post pages
// 	for post in posts {
// 		let mut destination = get_file("blog", &post.slug)?;

// 		write!(&mut destination, "{}", template!({
// 			header(&post.title, &post.description)}
// 			r#"<article itemscope itemtype="https://schema.org/BlogPosting">
// 				<header>
// 					<!-- TODO: Social Media image
// 					<img itemprop="image" src="" alt="">
// 					-->
// 					<h1 itemprop="headline">"# {post.title} r#"</h1>
// 					<span itemprop="author">Evan Brass</span>"#
// 					(move |fmt| {
// 						if let Some(published) = post.published {
// 							Display::fmt(&template!(
// 								r#"<time itemprop="datePublished" datetime=""#
// 									{published.format("%F")}
// 								r#"">"#
// 									{published.format("%B %-d, %Y")}
// 								"</time>"
// 							), fmt)
// 						} else {
// 							Ok(())
// 						}
// 					})
// 					r#"<ul itemprop="keywords">"#
// 						[post.tags.iter().map(|t| template!("<li>" {t.name} "</li>"))]
// 					r#"</ul>
// 					<nav is="blog-contents">
// 					<!-- TODO -->
// 					</nav>
// 				</header>
// 				<div itemprop="articleBody">"#
// 					{post.content}
// 				r#"</div>
// 			</article>"#
// 			{footer()}
// 		))?;
// 	}

// 	Ok(())
// }

fn main() -> Result<(), Box<dyn Error>> {
	let (tx, rx) = channel();
	let mut watcher = watcher(tx, Duration::from_secs(5))?;

	let conn = Connection::open("blog.sqlite")?;
	watcher.watch("blog.sqlite", RecursiveMode::NonRecursive)?;

	// TODO: Load templates and watch the template directory.
	let tera = Tera::new("templates/**/*.html")?;
	watcher.watch("templates", RecursiveMode::Recursive)?;

	let mut stmt = conn.prepare(r"
		SELECT * FROM posts
		LEFT JOIN posts_tags on post_id = posts_id
		LEFT JOIN tags on tags_id = tag_id
		ORDER BY post_published
	;")?;
	let posts = Post::distinct_from_rows(&mut stmt.query([])?)?;
	let posts = &posts;

	// Output the individual post pages
	for post in posts {
		let mut context = Context::new();
		context.insert("post", post);
		tera.render_to(
			"single.html",
			&context,
			get_file("blog", &post.slug)?
		)?;
	}

	loop {
		let ev = rx.recv()?;
		println!("{:?}", ev);
	}
}