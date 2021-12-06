use std::error::Error;

use rusqlite::Connection;

pub fn schema(conn: &Connection) -> Result<(), Box<dyn Error>> {
	conn.execute(
		"CREATE TABLE IF NOT EXISTS posts( \
			title TEXT NOT NULL UNIQUE, \
			slug TEXT GENERATE ALWAYS AS (slugify(strip_tags(title))) STORED, \
			published DATE NULL, \
			updated DATE GENERATE ALWAYS AS (date('now')) STORED, \
			description TEXT NULL, \
			keywords json NOT NULL DEFAULT (json_array()), \
			content TEXT NOT NULL DEFAULT '' \
		);",
		[],
	)?;

	Ok(())
}
