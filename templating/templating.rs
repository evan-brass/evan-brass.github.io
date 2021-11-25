use std::cell::RefCell;
pub use templating_macro::template;

pub struct Template<T>(pub RefCell<T>);
impl<T: FnMut(&mut std::fmt::Formatter<'_>) -> std::fmt::Result> std::fmt::Display for Template<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.0.borrow_mut()(f)
	}
}
