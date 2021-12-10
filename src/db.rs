use std::lazy::Lazy;
use std::path::Path;

use rusqlite::functions::FunctionFlags;
use rusqlite::Connection;
use voca_rs::manipulate::slugify;
use voca_rs::strip::strip_tags;

thread_local! {
	pub static CONNECTION: Lazy<Connection> = Lazy::new(|| {
		// Open the connection:
		let conn = Connection::open(Path::new("blog.sqlite")).unwrap();

		// Create our scalar functions:
		conn.create_scalar_function(
			"strip_tags",
			1,
			FunctionFlags::SQLITE_DETERMINISTIC | FunctionFlags::SQLITE_UTF8,
			|context| -> rusqlite::Result<String> { Ok(strip_tags(context.get_raw(0).as_str()?)) },
		).unwrap();
		conn.create_scalar_function(
			"slugify",
			1,
			FunctionFlags::SQLITE_DETERMINISTIC | FunctionFlags::SQLITE_UTF8,
			|context| -> rusqlite::Result<String> { Ok(slugify(context.get_raw(0).as_str()?)) },
		).unwrap();

		// Execute the schema:
		// updated DATE GENERATE ALWAYS AS (date('now')) STORED, \
		conn.execute(
			r"PRAGMA foreign_keys = ON;
			CREATE TABLE IF NOT EXISTS files(
				id INTEGER AUTOINCREMENT PRIMARY KEY,
				data BLOB NOT NULL,
				extension TEXT NULL,
				type TEXT NOT NULL
			);
			CREATE TABLE IF NOT EXISTS images (
				id INTEGER AUTOINCREMENT PRIMARY KEY,
				alt TEXT NOT NULL DEFAULT '',
				invert_dark BOOLEAN NOT NULL DEFAULT FALSE
			);
			CREATE TABLE IF NOT EXISTS image_source(
				width INTEGER NOT NULL,
				height INTEGER NOT NULL,
				media TEXT NULL,
				images_id INTEGER REFERENCES images(id) ON DELETE CASCADE,
				file_id INTEGER REFERENCES files(id) ON DELETE CASCADE
			);
			CREATE TABLE IF NOT EXISTS posts(
				title TEXT NOT NULL UNIQUE,
				slug TEXT GENERATE ALWAYS AS (slugify(strip_tags(title))) STORED,
				published DATE NULL,
				description TEXT NULL,
				keywords json NOT NULL DEFAULT (json_array()),
				social_image INTEGER NULL REFERENCES images(id) ON DELETE SET NULL,
				content TEXT NOT NULL DEFAULT ''
			);
			
			INSERT OR IGNORE INTO posts(title, published, description, keywords, content) VALUES (
				'Introduction to Programming 1: Background',
				date('2018-07-17'),
				'An introduction to computing.',
				json_array('Javascript', 'Tutorial'),
				'The content...'
			), (
				'Introduction to Programming 2: Getting started',
				date('2018-07-17'),
				'How to make your first few programs using Javascript from your browser.',
				json_array('Javascript', 'Tutorial'),
				'Another post...'
			), (
				'Introduction to Programming 3: Booleans, Logic, and Conditionals',
				date('2018-07-19'),
				'Booleans.',
				json_array('Javascript', 'Tutorial'),
				'Part 3'
			), (
				'Finite State Machines and Javascript',
				date('2019-07-24'),
				'Async functions are mapped to suspendable functions - state machines.',
				json_array('Javascript', 'Finite State Machines', 'Async', 'Frameworkless'),
				'FSMs in JS using async functions'
			), (
				'Distributed Web Applications',
				date('2020-03-27'),
				'TODO',
				json_array('Javascript', 'Distributed Systems', 'WebRTC', 'WebPush'),
				'TODO'
			), (
				'Built-In and Custom Traits in Javascript',
				date('2020-06-15'),
				'TODO',
				json_array('Javascript', 'Patterns', 'Traits'),
				'TODO'
			), (
				'How to Instantiate HTML Template Elements',
				date('2020-08-10'),
				'TODO',
				json_array('Javascript', 'Templates'),
				'TODO'
			), (
				'How to Parse HTML with Regular Expressions',
				date('2020-12-25'),
				'TODO',
				json_array('Javascript', 'Parsing', 'HTML'),
				'TODO'
			), (
				'The Most Cursed Javascript',
				date('2021-03-21'),
				'TODO',
				json_array('Javascript', 'Cursed', 'Bad Practice'),
				'TODO'
			), (
				'Turn Your Rockets into Washing Machines',
				date('2021-05-20'),
				'TODO',
				json_array('Javascript', 'Finite State Machines', 'Async'),
				'TODO'
			), (
				'My Bare Metal RPi Journey',
				date('2021-07-30'),
				'TODO',
				json_array('Rust', 'Bare Metal', 'RaspberryPi'),
				'TODO'
			);",
			[],
		).unwrap();

		conn
	});
}
