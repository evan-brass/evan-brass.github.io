use rusqlite::{Row, Rows};
use serde::Serialize;
use chrono::NaiveDate;

#[derive(Debug, Serialize)]
pub struct Tag {
	pub name: String,
	pub slug: String
}
impl Tag {
	pub fn try_from_row(row: &Row) -> rusqlite::Result<Self> {
		let name = row.get("tag_name")?;
		let slug = row.get("tag_name")?;
		Ok(Self { name, slug })
	}
}

#[derive(Debug, Serialize)]
pub struct Post {
	pub id: u32,
	pub title: String,
	pub slug: String,
	pub published: Option<NaiveDate>,
	pub description: String,
	pub tags: Vec<Tag>,
	pub content: String
}
impl Post {
	pub fn try_from_row(row: &Row) -> rusqlite::Result<Self> {
		let id = row.get("post_id")?;
		let title = row.get("post_title")?;
		let slug = row.get("post_slug")?;
		let published = row.get("post_published")?;
		let description = row.get("post_description")?;
		let mut tags = Vec::new();
		if let Ok(tag) = Tag::try_from_row(row) {
			tags.push(tag);
		}
		let content = row.get("post_content")?;

		Ok(Self { id, title, slug, published, description, tags, content })
	}
	pub fn distinct_from_rows(rows: &mut Rows) -> rusqlite::Result<Vec<Self>> {
		let mut ret: Vec<Post> = Vec::new();

		while let Some(row) = rows.next()? {
			let id: u32 = row.get("post_id")?;

			match ret.last_mut() {
				Some(post) if post.id == id => {
					post.tags.push(Tag::try_from_row(row)?);
				},
				_ => { ret.push(Post::try_from_row(row)?); }
			}
		}

		Ok(ret)
	}
}