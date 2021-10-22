use std::str::pattern::Pattern;

trait StrExt<'i> {
	fn take<P: Pattern<'i>>(&mut self, p: P) -> Option<&'i Self>;
}
impl<'i> StrExt<'i> for str {
	fn take<P: Pattern<'i>>(&mut self, p: P) -> Option<&'i Self> {
		None
	}
}