use crate::post::Post;

const HEADER_STYLES: &'static str = include_str!("header-styles.css");

fn page_header<T: std::fmt::Write>(out: &mut T, title: &str, description: &str) -> std::fmt::Result {
	writeln!(out, 
r#"<!DOCTYPE html>
<html lang="en">
<head>
		<meta charset="utf-8">
		<meta name="viewport" content="width=device-width, initial-scale=1">
		<title>{}</title>
		<meta name="description" content="{}">
		<style>{}</style>
		<link rel="preload" as="style" href="/css/main.css" onload="this.onload=null;this.rel='stylesheet'">
		<link rel="stylesheet" media="print" href="/css/print.css">
</head>
<body>
	<header>
		<h1>Evan Brass</h1>
		<nav>
			<a href="/">Home</a>
			<a href="/about/">About</a>
			<a href="/blog/">Blog</a>
		</nav>
	</header>
	<main>"#, title, description, HEADER_STYLES)?;

	Ok(())
}

pub fn post_single<T: std::fmt::Write>(out: &mut T, post: &Post) -> std::fmt::Result {
	page_header(out, &post.title, &post.description)?;
	writeln!(out, 
r#"		<article itemscope itemtype="https://schema.org/BlogPosting">
			<header>
				<!-- TODO: Social Media image
				<img itemprop="image" src="" alt="">
				-->
				<h1 itemprop="headline">{}</h1>
				<span itemprop="author">Evan Brass</span>"#, post.title)?;
	if let Some(published) = post.published {
		writeln!(out,
r#"				<time itemprop="datePublished" datetime="{}">{}</time>"#, published.format("%F"), published.format("%B %-d, %Y"))?;
	}
	writeln!(out,
r#"				<ul itemprop="keywords">"#,
	)?;
	for tag in post.tags.iter() {
		write!(out, r#"<li>{}</li>"#, tag.name)?;
	}
	write!(out, r#"</ul>
		<nav is="blog-contents">
			<!-- TODO -->
		</nav>
	</header>
	<div itemprop="articleBody">
		{}
	</div>
</article>"#, post.content)?;
	page_footer(out)
}

pub  fn post_list<T: std::fmt::Write>(out: &mut T, posts: &Vec<Post>) -> std::fmt::Result {
	page_header(out, "", "")?;
	writeln!(out, "<ul>")?;
	for post in posts.iter() {
		writeln!(out, 
r#"		<li>
			<h2>{}</h2>
			<time>{}</time>
			<p>{}</p>
		</li>"#, post.title, post.published.map(|d| d.format("%B %-d, %Y").to_string()).unwrap_or("".into()), post.description)?;
	}
	writeln!(out, "</ul>")?;
	page_footer(out)
}

fn page_footer<T: std::fmt::Write>(out: &mut T) -> std::fmt::Result {
	write!(out, 
r#"
	</main>
	<footer>
		<a href="https://twitter.com/evan_brass">Twitter</a> and <a href="https://github.com/evan-brass">GitHub</a>
	</footer>
	<noscript>
		<link rel="stylesheet" href="/css/main.css">
	</noscript>
</body>
</html>"#)?;

	Ok(())
}