#![feature(option_result_contains, path_try_exists, if_let_guard, pattern)]
use std::{io::Write, fs::{
	self, File
}, io, path::{PathBuf, Path}};

mod parser;
mod parser2;

fn render_document(mut output: File, contents: &str) -> io::Result<()> {
	let (header, _) = parser::parse_header(&contents);
	println!("{:?}", header);
	let o = &mut output;
	write!(o, 
r#"<!DOCTYPE html>
<html lang="en">
	<head>
		<meta charset="UTF-8">
		<title>{}</title>
		<meta name="description" content="{}">
		<meta name="keywords" content="">
	</head>
	<body>
		<header>
			<h1>Evan Brass</h1>
			<p>A mediocre blog about stuff</p>
			<nav>
				<a href="/about/">About</a>
				<a href="/blog/">Blog</a>
				<a href="/projects/">Projects</a>
			</nav>
		</header>
		<main>"#, header.title, header.description)?;

	
	write!(o, r#"
		</main>
		<footer>

		</footer>
	</body>
</html>"#)?;
	Ok(())
}

struct Site {
	src: PathBuf,
	dest: PathBuf
}
impl Site {
	fn handle_dir(&self, dir: &Path) -> io::Result<()> {
		// TODO: Need to be able to sort the index.
		let _index: Vec<String> = Vec::new();
		for entry in fs::read_dir(dir)?.into_iter() {
			let entry = entry?;
			let ft = entry.file_type()?;
			let path = entry.path();
			if ft.is_dir() {
				self.handle_dir(&path)?;
			} else if ft.is_file() {
				let mut dest;
				if path.extension().contains(&"md") {
					if path.file_name().unwrap() != "index.md" {
						dest = self.dest.join(path.strip_prefix(&self.src).unwrap());
						dest.set_extension("");
						dest.push("index.html");
					} else {
						dest = self.dest.join(path.strip_prefix(&self.src).unwrap());
						dest.set_extension("html");
					}
				} else {
					dest = self.dest.join(path.strip_prefix(&self.src).unwrap());
				}
				println!("{:?}", dest);

				// Create the parent directory if it doesn't already exist.
				if !dest.parent().unwrap().try_exists()? {
					fs::create_dir_all(dest.parent().unwrap())?;
				}
				if path.extension().contains(&"md") {
					// Render the document
					let contents = fs::read_to_string(path)?;
					let post = std::fs::File::create(dest)?;
					render_document(post, &contents)?;
				} else {
					fs::copy(path, dest)?;
				}
			} else {
				panic!("Symlinks are not allowed in the content directory: {:?}", path);
			}
		}

		Ok(())
	}
	pub fn build(self) -> io::Result<()> {
		self.handle_dir(&self.src)?;

		Ok(())
	}
}
impl Default for Site {
	fn default() -> Self {
		Self { src: PathBuf::from("content"), dest: PathBuf::from("public") }
	}
}


fn main() -> io::Result<()> {
	Site::default().build()?;

	Ok(())
}
