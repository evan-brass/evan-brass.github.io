#![feature(proc_macro_quote)]
use proc_macro::{Delimiter, TokenStream, TokenTree, quote};

#[proc_macro]
pub fn template(input: TokenStream) -> TokenStream {
	let steps: TokenStream = input.into_iter().map(|t| match t {
		// Literals (like strings mostly):
		TokenTree::Literal(_) => quote!(::std::fmt::Display::fmt(&$t, fmt)?;),
		// Lists:
		TokenTree::Group(g) if g.delimiter() == Delimiter::Bracket => {
			let list = g.stream();
			quote!(for ref x in ($list) {
				::std::fmt::Display::fmt(&x, fmt)?;
			})
		},
		// Normal Expressions:
		TokenTree::Group(g) if g.delimiter() == Delimiter::Brace => {
			let e = g.stream();
			quote!(::std::fmt::Display::fmt(&$e, fmt)?;)
		},
		// Non-dispayed expressions (they can use the fmt variable.):
		TokenTree::Group(ref g) if g.delimiter() == Delimiter::Parenthesis => {
			quote!($t(fmt)?;)
		}
		_ => panic!("This type of TokenTree isn't valid inside the template! macro.")
	}).collect();
	quote!(::templating::Template(::std::cell::RefCell::new(move |fmt: &mut ::std::fmt::Formatter<'_>| {
		$steps
		Ok(())
	})))
}
