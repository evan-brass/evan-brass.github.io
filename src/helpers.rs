use std::path::Path;

use chrono::NaiveDate;
use rocket_dyn_templates::handlebars::*;

pub fn fmtdate(
	h: &Helper<'_, '_>,
	_: &Handlebars<'_>,
	_: &Context,
	_: &mut RenderContext<'_, '_>,
	out: &mut dyn Output,
) -> HelperResult {
	let dstr = h
		.param(0)
		.and_then(|v| v.value().as_str())
		.ok_or(RenderError::new(
			"Must provide a string for the first argument.",
		))?;
	let din = NaiveDate::parse_from_str(dstr, "%F")
		.map_err(|e| RenderError::from_error("date_parse_error", e))?;
	let fmtstr = h
		.param(1)
		.and_then(|v| v.value().as_str())
		.unwrap_or("%B %-d, %Y");
	out.write(&din.format(fmtstr).to_string())?;
	Ok(())
}

pub fn includestatic(
	h: &Helper<'_, '_>,
	_: &Handlebars<'_>,
	_: &Context,
	_: &mut RenderContext<'_, '_>,
	out: &mut dyn Output,
) -> HelperResult {
	let file = h
		.param(0)
		.and_then(|v| v.value().as_str())
		.map(Path::new)
		.ok_or(RenderError::new("Must provide a file name."))?;
	let s = std::fs::read_to_string(Path::new("static").join(file))?;
	out.write(&s)?;
	Ok(())
}