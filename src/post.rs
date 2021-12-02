use rusqlite::Row;
use rusqlite::types::FromSqlError;
use serde::Serialize;
use chrono::NaiveDate;

#[derive(Debug, Serialize)]
pub struct Post {
	pub title: String,
	pub slug: String,
	pub published: Option<NaiveDate>,
	pub description: Option<String>,
	pub keywords: Vec<String>,
	pub content: String
}
impl Post {
	pub fn try_from_row(row: &Row) -> rusqlite::Result<Self> {
		let title = row.get("title")?;
		let published = row.get("published")?;
		let description = row.get("description")?;
		let keywords = row.get_ref("keywords")?.as_str()?;
		let keywords = serde_json::from_str(keywords).map_err(|e| FromSqlError::Other(e.into()))?;
		let slug = row.get("slug")?;
		let content = row.get("content")?;

		Ok(Self { title, published, description, keywords, slug, content })
	}
}