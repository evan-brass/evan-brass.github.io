use std::cell::RefCell;
use std::collections::HashSet;
use std::lazy::SyncLazy;
use std::ops::DerefMut;
use std::sync::Mutex;

use rusqlite::types::ValueRef;
use rusqlite::Row;
use rusqlite::{CachedStatement, Connection, Params, Statement};
use serde::{ser::Error as _, ser::SerializeSeq as _, ser::SerializeStruct, Serialize, Serializer};

static COLUMN_NAMES: SyncLazy<Mutex<HashSet<&'static str>>> =
	SyncLazy::new(|| Mutex::new(HashSet::new()));

pub enum ColumnType {
	// TODO: Dates and shit
	None,
	Json,
	Other(String),
}
impl From<Option<&str>> for ColumnType {
	fn from(s: Option<&str>) -> Self {
		match s {
			None => ColumnType::None,
			Some("json") => ColumnType::Json,
			Some(t) => ColumnType::Other(t.to_owned()),
		}
	}
}
fn get_columns(stmt: &Statement) -> Vec<(&'static str, ColumnType)> {
	stmt.columns()
		.into_iter()
		.map(|c| {
			(
				*COLUMN_NAMES
					.lock()
					.unwrap()
					.get_or_insert_with(c.name(), |n| Box::leak(n.to_owned().into_boxed_str())),
				From::from(c.decl_type()),
			)
		})
		.collect()
}

pub struct RowWrapper<'a, C: AsRef<[(&'static str, ColumnType)]>> {
	columns: C,
	row: &'a Row<'a>,
}
impl<'a> RowWrapper<'a, Vec<(&'static str, ColumnType)>> {
	pub fn new(row: &'a Row<'a>) -> Self {
		let columns = get_columns(row.as_ref());
		Self { columns, row }
	}
}
impl<'stmt, C: AsRef<[(&'static str, ColumnType)]>> Serialize for RowWrapper<'stmt, C> {
	fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		// TODO: Let the struct be named
		// TODO: Date and DateTime types
		let mut struc = serializer.serialize_struct("row", self.columns.as_ref().len())?;
		for (key, decl) in self.columns.as_ref().iter() {
			let vr = self.row.get_ref(*key).map_err(|e| S::Error::custom(e))?;
			match (vr, decl) {
				(ValueRef::Null, _) => struc.skip_field(*key)?,
				(ValueRef::Integer(ref i), _) => struc.serialize_field(*key, i)?,
				(ValueRef::Real(ref r), _) => struc.serialize_field(*key, r)?,
				(ValueRef::Text(s), ColumnType::Json) => struc.serialize_field(
					*key,
					&serde_json::from_slice::<serde_json::Value>(s)
						.map_err(|e| S::Error::custom(e))?,
				)?,
				(ValueRef::Text(s), _) => struc.serialize_field(
					*key,
					std::str::from_utf8(s).map_err(|e| S::Error::custom(e))?,
				)?,
				(ValueRef::Blob(b), _) => struc.serialize_field(*key, b)?,
			}
		}
		struc.end()
	}
}

pub struct Query<'conn, D: DerefMut<Target = Statement<'conn>>> {
	stmt: RefCell<D>,
}
impl<'conn, D: DerefMut<Target = Statement<'conn>>> Query<'conn, D> {
	pub fn new(stmt: D) -> Self {
		Self {
			stmt: RefCell::new(stmt),
		}
	}
}
impl<'conn> Query<'conn, CachedStatement<'conn>> {
	pub fn new_sql<P: Params>(conn: &'conn Connection, sql: &'static str, params: P) -> Self {
		let mut stmt = conn.prepare_cached(sql).unwrap();
		params.__bind_in(&mut stmt).unwrap();
		Self {
			stmt: RefCell::new(stmt),
		}
	}
}
impl<'conn, D: DerefMut<Target = Statement<'conn>>> Serialize for Query<'conn, D> {
	fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		let mut stmt = self.stmt.borrow_mut();
		let stmt = stmt.deref_mut();
		let columns = get_columns(stmt);
		let mut rows = stmt.raw_query();

		let mut seq = serializer.serialize_seq(None)?;
		while let Some(row) = rows.next().map_err(|e| S::Error::custom(e))? {
			seq.serialize_element(&RowWrapper {
				columns: &columns,
				row,
			})?;
		}
		seq.end()
	}
}
