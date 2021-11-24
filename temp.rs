#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use std::path::{Path, PathBuf};
use std::fmt::Display;
use templating::template;
#[macro_use]
extern crate rocket;
use rocket::fs::NamedFile;
use rocket::response::content::Html;
use rocket_sync_db_pools::{
    database,
    rusqlite::{self, Connection},
};
use rocket_dyn_templates::Template;
mod post {
    use rocket_sync_db_pools::{
        rusqlite::{self, Row, Rows},
    };
    use serde::Serialize;
    use chrono::NaiveDate;
    pub struct Tag {
        pub name: String,
        pub slug: String,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for Tag {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                Tag {
                    name: ref __self_0_0,
                    slug: ref __self_0_1,
                } => {
                    let debug_trait_builder = &mut ::core::fmt::Formatter::debug_struct(f, "Tag");
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "name",
                        &&(*__self_0_0),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "slug",
                        &&(*__self_0_1),
                    );
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Tag {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "Tag",
                    false as usize + 1 + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "name",
                    &self.name,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "slug",
                    &self.slug,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    impl Tag {
        pub fn try_from_row(row: &Row) -> rusqlite::Result<Self> {
            let name = row.get("tag_name")?;
            let slug = row.get("tag_name")?;
            Ok(Self { name, slug })
        }
    }
    pub struct Post {
        pub id: u32,
        pub title: String,
        pub slug: String,
        pub published: Option<NaiveDate>,
        pub description: String,
        pub tags: Vec<Tag>,
        pub content: String,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for Post {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                Post {
                    id: ref __self_0_0,
                    title: ref __self_0_1,
                    slug: ref __self_0_2,
                    published: ref __self_0_3,
                    description: ref __self_0_4,
                    tags: ref __self_0_5,
                    content: ref __self_0_6,
                } => {
                    let debug_trait_builder = &mut ::core::fmt::Formatter::debug_struct(f, "Post");
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder, "id", &&(*__self_0_0));
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "title",
                        &&(*__self_0_1),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "slug",
                        &&(*__self_0_2),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "published",
                        &&(*__self_0_3),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "description",
                        &&(*__self_0_4),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "tags",
                        &&(*__self_0_5),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "content",
                        &&(*__self_0_6),
                    );
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Post {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "Post",
                    false as usize + 1 + 1 + 1 + 1 + 1 + 1 + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "id",
                    &self.id,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "title",
                    &self.title,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "slug",
                    &self.slug,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "published",
                    &self.published,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "description",
                    &self.description,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "tags",
                    &self.tags,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "content",
                    &self.content,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
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
            Ok(Self {
                id,
                title,
                slug,
                published,
                description,
                tags,
                content,
            })
        }
        pub fn distinct_from_rows(rows: &mut Rows) -> rusqlite::Result<Vec<Self>> {
            let mut ret: Vec<Post> = Vec::new();
            while let Some(row) = rows.next()? {
                let id: u32 = row.get("post_id")?;
                if let Some(post) = ret.iter_mut().find(|p| id == p.id) {
                    if let Ok(tag) = Tag::try_from_row(row) {
                        post.tags.push(tag);
                    }
                } else {
                    ret.push(Post::try_from_row(row)?);
                }
            }
            Ok(ret)
        }
    }
}
use post::Post;
struct BlogDB(::rocket_sync_db_pools::Connection<Self, Connection>);
impl BlogDB {
    /// Returns a fairing that initializes the associated database
    /// connection pool.
    pub fn fairing() -> impl ::rocket_sync_db_pools::rocket::fairing::Fairing {
        <::rocket_sync_db_pools::ConnectionPool<Self, Connection>>::fairing(
            "\'sqlite_blog\' Database Pool",
            "sqlite_blog",
        )
    }
    /// Retrieves a connection of type `Self` from the `rocket`
    /// instance. Returns `Some` as long as `Self::fairing()` has been
    /// attached.
    pub async fn get_one<P>(__rocket: &::rocket_sync_db_pools::rocket::Rocket<P>) -> Option<Self>
    where
        P: ::rocket_sync_db_pools::rocket::Phase,
    {
        <::rocket_sync_db_pools::ConnectionPool<Self, Connection>>::get_one(&__rocket)
            .await
            .map(Self)
    }
    /// Runs the provided closure on a thread from a threadpool. The
    /// closure will be passed an `&mut r2d2::PooledConnection`.
    /// `.await`ing the return value of this function yields the value
    /// returned by the closure.
    pub async fn run<F, R>(&self, __f: F) -> R
    where
        F: FnOnce(&mut Connection) -> R + Send + 'static,
        R: Send + 'static,
    {
        self.0.run(__f).await
    }
}
impl<'r> ::rocket_sync_db_pools::rocket::request::FromRequest<'r> for BlogDB {
    type Error = ();
    #[allow(
        clippy::let_unit_value,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds,
        clippy::used_underscore_binding
    )]
    fn from_request<'life0, 'async_trait>(
        __r: &'r ::rocket_sync_db_pools::rocket::request::Request<'life0>,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<
                    Output = ::rocket_sync_db_pools::rocket::request::Outcome<Self, ()>,
                > + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'r: 'async_trait,
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                ::rocket_sync_db_pools::rocket::request::Outcome<Self, ()>,
            > {
                return __ret;
            }
            let __r = __r;
            let __ret: ::rocket_sync_db_pools::rocket::request::Outcome<Self, ()> = {
                <::rocket_sync_db_pools::Connection<Self, Connection>>::from_request(__r)
                    .await
                    .map(Self)
            };
            #[allow(unreachable_code)]
            __ret
        })
    }
}
impl ::rocket_sync_db_pools::rocket::Sentinel for BlogDB {
    fn abort(
        __r: &::rocket_sync_db_pools::rocket::Rocket<::rocket_sync_db_pools::rocket::Ignite>,
    ) -> bool {
        <::rocket_sync_db_pools::Connection<Self, Connection>>::abort(__r)
    }
}
const HEADER_STYLES : & 'static str = "html {\n\tmargin: 0;\n\n\t/* Global Formatting */\n\ttab-size: 4;\n\tline-height: 2;\n\tfont-family: sans-serif;\n\tfont-size: 16px;\n\torphans: 2;\n\twidows: 2;\n}\npre {\n\tbackground-color: inherit;\n\twhite-space: pre-wrap;\n\tline-height: normal;\n\tfont-size: normal;\n\tpadding: 0.5em;\n\toverflow-wrap: break-word;\n}\nbody {\n\tmargin: 0 auto;\n\tpadding: 1em 2em;\n\tmax-width: 80ch;\n}\nul[itemprop=\"keywords\"] {\n\tpadding: 0;\n\tmargin: 0;\n}\nul[itemprop=\"keywords\"] li:not(:first-of-type)::before {\n\tcontent: \", \";\n}\nul[itemprop=\"keywords\"] li {\n\tdisplay: inline;\n}\nspan[itemprop=\"author\"] {\n\tdisplay: none;\n}\nh1, h2, h3, h4, h5, h6 {\n\tmargin-bottom: 0;\n\tfont-family: serif;\n\tbreak-inside: avoid;\n\tbreak-after: avoid;\n}\n:is(h1, h2, h3, h4, h5, h6) + :is(p, ol, ul, pre) {\n\tmargin-top: 0;\n}\np {\n\ttext-indent: 4ch;\n}\nbody > header {\n\tborder: 1px solid currentColor;\n\tborder-width: 0 1px;\n\tpadding: 1em 2em;\n\tdisplay: flex;\n\talign-items: baseline;\n\tjustify-content: space-between;\n}\nbody > header > h1 {\n\tletter-spacing: 0.2ch;\n}\nmain > ol {\n\tpadding: unset;\n}\nheader h1, header h2 {\n\tmargin: 0;\n}\nimg {\n\tmax-width: 100%;\n}\n@media (max-width: 80ch) {\n\thtml {\n\t\ttab-size: 2ch;\n\t}\n\tp {\n\t\ttext-indent: 2ch;;\n\t}\n}" ;
fn header<'a>(title: &'a str, description: &'a str) -> impl Display + 'a {
    ::templating::Template(::std::cell::RefCell::new(
        move |fmt: &mut ::std::fmt::Formatter<'_>| {
            ::std::fmt::Display::fmt(
                &r#"<!DOCTYPE html>
	<html lang="en">
	<head>
			<meta charset="utf-8">
			<meta name="viewport" content="width=device-width, initial-scale=1">
			<title>"#,
                fmt,
            )?;
            ::std::fmt::Display::fmt(&title, fmt)?;
            ::std::fmt::Display::fmt(
                &r#"</title>
			<meta name="description" content=""#,
                fmt,
            )?;
            ::std::fmt::Display::fmt(&description, fmt)?;
            ::std::fmt::Display::fmt(
                &r#"">
			<style>"#,
                fmt,
            )?;
            ::std::fmt::Display::fmt(&HEADER_STYLES, fmt)?;
            ::std::fmt::Display::fmt(
                &r#"</style>
			<link rel="preload" as="style" href="/css/main.css" onload="this.onload=null;this.rel='stylesheet'">
			<link rel="stylesheet" media="print" href="/css/print.css">
	</head>
	<body>
		<header>
			<h1>Evan Brass</h1>
			<nav>
				<a href="/">Home</a>
				<a href="/about/">About</a>
				<a href="/blog/">Blog</a>
			</nav>
		</header>
		<main>"#,
                fmt,
            )?;
            Ok(())
        },
    ))
}
fn footer() -> impl Display {
    ::templating::Template(::std::cell::RefCell::new(
        move |fmt: &mut ::std::fmt::Formatter<'_>| {
            ::std::fmt::Display::fmt(
                &r#"		</main>
	<footer>
		<a href="https://twitter.com/evan_brass">Twitter</a> and <a href="https://github.com/evan-brass">GitHub</a>
	</footer>
	<noscript>
		<link rel="stylesheet" href="/css/main.css">
	</noscript>
</body>
</html>"#,
                fmt,
            )?;
            Ok(())
        },
    ))
}
async fn index(db: BlogDB) -> Html<String> {
    let posts = db
        .run(|conn| -> rusqlite::Result<Vec<Post>> {
            let mut stmt = conn.prepare(
                r"SELECT * FROM posts
			LEFT JOIN posts_tags ON post_id = posts_id
			LEFT JOIN tags on tags_id = tag_id
			ORDER BY post_published
			;",
            )?;
            let posts = Post::distinct_from_rows(&mut stmt.query([])?)?;
            Ok(posts)
        })
        .await
        .unwrap();
    Html(
        ::templating::Template(::std::cell::RefCell::new(
            move |fmt: &mut ::std::fmt::Formatter<'_>| {
                ::std::fmt::Display::fmt(&header("Evan Brass", "His life and times."), fmt)?;
                ::std::fmt::Display::fmt(&r"			<ol>", fmt)?;
                for ref x in (posts.iter().map(|p| {
                    ::templating::Template(::std::cell::RefCell::new(
                        move |fmt: &mut ::std::fmt::Formatter<'_>| {
                            ::std::fmt::Display::fmt(
                                &r"<li>
					<h2>",
                                fmt,
                            )?;
                            ::std::fmt::Display::fmt(&p.title, fmt)?;
                            ::std::fmt::Display::fmt(
                                &r"</h2>
					<p>",
                                fmt,
                            )?;
                            ::std::fmt::Display::fmt(&p.description, fmt)?;
                            ::std::fmt::Display::fmt(
                                &r"</p>
				</li>",
                                fmt,
                            )?;
                            Ok(())
                        },
                    ))
                })) {
                    ::std::fmt::Display::fmt(&x, fmt)?;
                }
                ::std::fmt::Display::fmt(&r#"			</ol>"#, fmt)?;
                ::std::fmt::Display::fmt(&footer(), fmt)?;
                Ok(())
            },
        ))
        .to_string(),
    )
}
#[doc(hidden)]
#[allow(non_camel_case_types)]
/// Rocket code generated proxy structure.
struct index {}
/// Rocket code generated proxy static conversion implementations.
impl index {
    #[allow(non_snake_case, unreachable_patterns, unreachable_code)]
    fn into_info(self) -> ::rocket::route::StaticInfo {
        fn monomorphized_function<'__r>(
            __req: &'__r ::rocket::request::Request<'_>,
            __data: ::rocket::data::Data<'__r>,
        ) -> ::rocket::route::BoxFuture<'__r> {
            ::std::boxed::Box::pin(async move {
                let __rocket_db: BlogDB =
                    match <BlogDB as ::rocket::request::FromRequest>::from_request(__req).await {
                        ::rocket::outcome::Outcome::Success(__v) => __v,
                        ::rocket::outcome::Outcome::Forward(_) => {
                            {
                                let lvl = ::log::Level::Warn;
                                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                                    ::log::__private_api_log(
                                        ::core::fmt::Arguments::new_v1(
                                            &["`", "` request guard is forwarding."],
                                            &match (&"BlogDB",) {
                                                (arg0,) => [::core::fmt::ArgumentV1::new(
                                                    arg0,
                                                    ::core::fmt::Display::fmt,
                                                )],
                                            },
                                        ),
                                        lvl,
                                        &("_", "blogger", "src/main.rs", 59u32),
                                    );
                                }
                            };
                            return ::rocket::outcome::Outcome::Forward(__data);
                        }
                        ::rocket::outcome::Outcome::Failure((__c, __e)) => {
                            {
                                let lvl = ::log::Level::Warn;
                                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                                    ::log::__private_api_log(
                                        ::core::fmt::Arguments::new_v1(
                                            &["`", "` request guard failed: ", "."],
                                            &match (&"BlogDB", &__e) {
                                                (arg0, arg1) => [
                                                    ::core::fmt::ArgumentV1::new(
                                                        arg0,
                                                        ::core::fmt::Display::fmt,
                                                    ),
                                                    ::core::fmt::ArgumentV1::new(
                                                        arg1,
                                                        ::core::fmt::Debug::fmt,
                                                    ),
                                                ],
                                            },
                                        ),
                                        lvl,
                                        &("_", "blogger", "src/main.rs", 59u32),
                                    );
                                }
                            };
                            return ::rocket::outcome::Outcome::Failure(__c);
                        }
                    };
                let ___responder = index(__rocket_db).await;
                ::rocket::route::Outcome::from(__req, ___responder)
            })
        }
        ::rocket::route::StaticInfo {
            name: "index",
            method: ::rocket::http::Method::Get,
            uri: "/",
            handler: monomorphized_function,
            format: ::std::option::Option::None,
            rank: ::std::option::Option::None,
            sentinels: <[_]>::into_vec(box [
                {
                    #[allow(unused_imports)]
                    use ::rocket::sentinel::resolution::{Resolve, DefaultSentinel as _};
                    ::rocket::sentinel::Sentry {
                        type_id: std::any::TypeId::of::<BlogDB>(),
                        type_name: std::any::type_name::<BlogDB>(),
                        parent: None,
                        location: ("src/main.rs", 59u32, 20u32),
                        specialized: Resolve::<BlogDB>::SPECIALIZED,
                        abort: Resolve::<BlogDB>::abort,
                    }
                },
                {
                    #[allow(unused_imports)]
                    use ::rocket::sentinel::resolution::{Resolve, DefaultSentinel as _};
                    ::rocket::sentinel::Sentry {
                        type_id: std::any::TypeId::of::<Html<String>>(),
                        type_name: std::any::type_name::<Html<String>>(),
                        parent: None,
                        location: ("src/main.rs", 59u32, 31u32),
                        specialized: Resolve::<Html<String>>::SPECIALIZED,
                        abort: Resolve::<Html<String>>::abort,
                    }
                },
                {
                    #[allow(unused_imports)]
                    use ::rocket::sentinel::resolution::{Resolve, DefaultSentinel as _};
                    ::rocket::sentinel::Sentry {
                        type_id: std::any::TypeId::of::<String>(),
                        type_name: std::any::type_name::<String>(),
                        parent: None.or(Some(std::any::TypeId::of::<Html<String>>())),
                        location: ("src/main.rs", 59u32, 36u32),
                        specialized: Resolve::<String>::SPECIALIZED,
                        abort: Resolve::<String>::abort,
                    }
                },
            ]),
        }
    }
    #[doc(hidden)]
    pub fn into_route(self) -> ::rocket::Route {
        self.into_info().into()
    }
}
#[doc(hidden)]
pub use rocket_uri_macro_index_14762511466739307096 as rocket_uri_macro_index;
async fn single_post(slug: String, db: BlogDB) -> Option<Html<String>> {
    let mut posts = db
        .run(move |conn| -> rusqlite::Result<Vec<Post>> {
            let mut stmt = conn
                .prepare(
                    r"SELECT * FROM posts
		LEFT JOIN posts_tags ON post_id = posts_id
		LEFT JOIN tags on tags_id = tag_id
		WHERE post_slug = ?1
		;",
                )
                .unwrap();
            let posts = Post::distinct_from_rows(&mut stmt.query([slug])?)?;
            Ok(posts)
        })
        .await
        .unwrap();
    if let Some(post) = posts.pop() {
        Some(Html(
            ::templating::Template(::std::cell::RefCell::new(
                move |fmt: &mut ::std::fmt::Formatter<'_>| {
                    ::std::fmt::Display::fmt(&header(&post.title, &post.description), fmt)?;
                    ::std::fmt::Display::fmt(
                        &r#"			<article itemscope itemtype="https://schema.org/BlogPosting">
				<header>
					<!-- TODO: Social Media image
					<img itemprop="image" src="" alt="">
					-->
					<h1 itemprop="headline">"#,
                        fmt,
                    )?;
                    ::std::fmt::Display::fmt(&post.title, fmt)?;
                    ::std::fmt::Display::fmt(
                        &r#"</h1>
					<span itemprop="author">Evan Brass</span>"#,
                        fmt,
                    )?;
                    (move |fmt| {
                        if let Some(published) = post.published {
                            Display::fmt(
                                &::templating::Template(::std::cell::RefCell::new(
                                    move |fmt: &mut ::std::fmt::Formatter<'_>| {
                                        ::std::fmt::Display::fmt(
                                            &r#"					<time itemprop="datePublished" datetime=""#,
                                            fmt,
                                        )?;
                                        ::std::fmt::Display::fmt(&published.format("%F"), fmt)?;
                                        ::std::fmt::Display::fmt(&">", fmt)?;
                                        ::std::fmt::Display::fmt(
                                            &published.format("%B %-d, %Y"),
                                            fmt,
                                        )?;
                                        ::std::fmt::Display::fmt(&"</time>", fmt)?;
                                        Ok(())
                                    },
                                )),
                                fmt,
                            )
                        } else {
                            Ok(())
                        }
                    })(fmt)?;
                    ::std::fmt::Display::fmt(&r#"					<ul itemprop="keywords">"#, fmt)?;
                    for ref x in (post.tags.iter().map(|t| {
                        ::templating::Template(::std::cell::RefCell::new(
                            move |fmt: &mut ::std::fmt::Formatter<'_>| {
                                ::std::fmt::Display::fmt(&"<li>", fmt)?;
                                ::std::fmt::Display::fmt(&t.name, fmt)?;
                                ::std::fmt::Display::fmt(&"</li>", fmt)?;
                                Ok(())
                            },
                        ))
                    })) {
                        ::std::fmt::Display::fmt(&x, fmt)?;
                    }
                    ::std::fmt::Display::fmt(
                        &r#"					</ul>
					<nav is="blog-contents">
					<!-- TODO -->
					</nav>
				</header>
				<div itemprop="articleBody">"#,
                        fmt,
                    )?;
                    ::std::fmt::Display::fmt(&post.content, fmt)?;
                    ::std::fmt::Display::fmt(
                        &r#"				</div>
			</article>"#,
                        fmt,
                    )?;
                    ::std::fmt::Display::fmt(&footer(), fmt)?;
                    Ok(())
                },
            ))
            .to_string(),
        ))
    } else {
        None
    }
}
#[doc(hidden)]
#[allow(non_camel_case_types)]
/// Rocket code generated proxy structure.
struct single_post {}
/// Rocket code generated proxy static conversion implementations.
impl single_post {
    #[allow(non_snake_case, unreachable_patterns, unreachable_code)]
    fn into_info(self) -> ::rocket::route::StaticInfo {
        fn monomorphized_function<'__r>(
            __req: &'__r ::rocket::request::Request<'_>,
            __data: ::rocket::data::Data<'__r>,
        ) -> ::rocket::route::BoxFuture<'__r> {
            ::std::boxed::Box::pin(async move {
                let __rocket_db: BlogDB =
                    match <BlogDB as ::rocket::request::FromRequest>::from_request(__req).await {
                        ::rocket::outcome::Outcome::Success(__v) => __v,
                        ::rocket::outcome::Outcome::Forward(_) => {
                            {
                                let lvl = ::log::Level::Warn;
                                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                                    ::log::__private_api_log(
                                        ::core::fmt::Arguments::new_v1(
                                            &["`", "` request guard is forwarding."],
                                            &match (&"BlogDB",) {
                                                (arg0,) => [::core::fmt::ArgumentV1::new(
                                                    arg0,
                                                    ::core::fmt::Display::fmt,
                                                )],
                                            },
                                        ),
                                        lvl,
                                        &("_", "blogger", "src/main.rs", 83u32),
                                    );
                                }
                            };
                            return ::rocket::outcome::Outcome::Forward(__data);
                        }
                        ::rocket::outcome::Outcome::Failure((__c, __e)) => {
                            {
                                let lvl = ::log::Level::Warn;
                                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                                    ::log::__private_api_log(
                                        ::core::fmt::Arguments::new_v1(
                                            &["`", "` request guard failed: ", "."],
                                            &match (&"BlogDB", &__e) {
                                                (arg0, arg1) => [
                                                    ::core::fmt::ArgumentV1::new(
                                                        arg0,
                                                        ::core::fmt::Display::fmt,
                                                    ),
                                                    ::core::fmt::ArgumentV1::new(
                                                        arg1,
                                                        ::core::fmt::Debug::fmt,
                                                    ),
                                                ],
                                            },
                                        ),
                                        lvl,
                                        &("_", "blogger", "src/main.rs", 83u32),
                                    );
                                }
                            };
                            return ::rocket::outcome::Outcome::Failure(__c);
                        }
                    };
                let __rocket_slug: String = match __req.routed_segment(1usize) {
                    ::std::option::Option::Some(__s) => {
                        match <String as ::rocket::request::FromParam>::from_param(__s) {
                            ::std::result::Result::Ok(__v) => __v,
                            ::std::result::Result::Err(__error) => {
                                return {
                                    {
                                        let lvl = ::log::Level::Warn;
                                        if lvl <= ::log::STATIC_MAX_LEVEL
                                            && lvl <= ::log::max_level()
                                        {
                                            :: log :: __private_api_log (:: core :: fmt :: Arguments :: new_v1 (& ["`" , ": " , "` param guard parsed forwarding with error "] , & match (& "slug" , & "String" , & __error) { (arg0 , arg1 , arg2) => [:: core :: fmt :: ArgumentV1 :: new (arg0 , :: core :: fmt :: Display :: fmt) , :: core :: fmt :: ArgumentV1 :: new (arg1 , :: core :: fmt :: Display :: fmt) , :: core :: fmt :: ArgumentV1 :: new (arg2 , :: core :: fmt :: Debug :: fmt)] , }) , lvl , & ("_" , "blogger" , "src/main.rs" , 82u32)) ;
                                        }
                                    };
                                    ::rocket::outcome::Outcome::Forward(__data)
                                }
                            }
                        }
                    }
                    ::std::option::Option::None => {
                        {
                            let lvl = ::log::Level::Error;
                            if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                                ::log::__private_api_log(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Internal invariant broken: dyn param not found."],
                                        &match () {
                                            () => [],
                                        },
                                    ),
                                    lvl,
                                    &("_", "blogger", "src/main.rs", 83u32),
                                );
                            }
                        };
                        {
                            let lvl = ::log::Level::Error;
                            if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                                ::log::__private_api_log(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Please report this to the Rocket issue tracker."],
                                        &match () {
                                            () => [],
                                        },
                                    ),
                                    lvl,
                                    &("_", "blogger", "src/main.rs", 83u32),
                                );
                            }
                        };
                        {
                            let lvl = ::log::Level::Error;
                            if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                                ::log::__private_api_log(
                                    ::core::fmt::Arguments::new_v1(
                                        &["https://github.com/SergioBenitez/Rocket/issues"],
                                        &match () {
                                            () => [],
                                        },
                                    ),
                                    lvl,
                                    &("_", "blogger", "src/main.rs", 83u32),
                                );
                            }
                        };
                        return ::rocket::outcome::Outcome::Forward(__data);
                    }
                };
                let ___responder = single_post(__rocket_slug, __rocket_db).await;
                ::rocket::route::Outcome::from(__req, ___responder)
            })
        }
        ::rocket::route::StaticInfo {
            name: "single_post",
            method: ::rocket::http::Method::Get,
            uri: "/blog/<slug>",
            handler: monomorphized_function,
            format: ::std::option::Option::None,
            rank: ::std::option::Option::None,
            sentinels: <[_]>::into_vec(box [
                {
                    #[allow(unused_imports)]
                    use ::rocket::sentinel::resolution::{Resolve, DefaultSentinel as _};
                    ::rocket::sentinel::Sentry {
                        type_id: std::any::TypeId::of::<String>(),
                        type_name: std::any::type_name::<String>(),
                        parent: None,
                        location: ("src/main.rs", 83u32, 28u32),
                        specialized: Resolve::<String>::SPECIALIZED,
                        abort: Resolve::<String>::abort,
                    }
                },
                {
                    #[allow(unused_imports)]
                    use ::rocket::sentinel::resolution::{Resolve, DefaultSentinel as _};
                    ::rocket::sentinel::Sentry {
                        type_id: std::any::TypeId::of::<BlogDB>(),
                        type_name: std::any::type_name::<BlogDB>(),
                        parent: None,
                        location: ("src/main.rs", 83u32, 40u32),
                        specialized: Resolve::<BlogDB>::SPECIALIZED,
                        abort: Resolve::<BlogDB>::abort,
                    }
                },
                {
                    #[allow(unused_imports)]
                    use ::rocket::sentinel::resolution::{Resolve, DefaultSentinel as _};
                    ::rocket::sentinel::Sentry {
                        type_id: std::any::TypeId::of::<Option<Html<String>>>(),
                        type_name: std::any::type_name::<Option<Html<String>>>(),
                        parent: None,
                        location: ("src/main.rs", 83u32, 51u32),
                        specialized: Resolve::<Option<Html<String>>>::SPECIALIZED,
                        abort: Resolve::<Option<Html<String>>>::abort,
                    }
                },
                {
                    #[allow(unused_imports)]
                    use ::rocket::sentinel::resolution::{Resolve, DefaultSentinel as _};
                    ::rocket::sentinel::Sentry {
                        type_id: std::any::TypeId::of::<Html<String>>(),
                        type_name: std::any::type_name::<Html<String>>(),
                        parent: None.or(Some(std::any::TypeId::of::<Option<Html<String>>>())),
                        location: ("src/main.rs", 83u32, 58u32),
                        specialized: Resolve::<Html<String>>::SPECIALIZED,
                        abort: Resolve::<Html<String>>::abort,
                    }
                },
                {
                    #[allow(unused_imports)]
                    use ::rocket::sentinel::resolution::{Resolve, DefaultSentinel as _};
                    ::rocket::sentinel::Sentry {
                        type_id: std::any::TypeId::of::<String>(),
                        type_name: std::any::type_name::<String>(),
                        parent: None.or(Some(std::any::TypeId::of::<Html<String>>())),
                        location: ("src/main.rs", 83u32, 63u32),
                        specialized: Resolve::<String>::SPECIALIZED,
                        abort: Resolve::<String>::abort,
                    }
                },
            ]),
        }
    }
    #[doc(hidden)]
    pub fn into_route(self) -> ::rocket::Route {
        self.into_info().into()
    }
}
#[doc(hidden)]
pub use rocket_uri_macro_single_post_16718862082445222703 as rocket_uri_macro_single_post;
async fn static_files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).await.ok()
}
#[doc(hidden)]
#[allow(non_camel_case_types)]
/// Rocket code generated proxy structure.
struct static_files {}
/// Rocket code generated proxy static conversion implementations.
impl static_files {
    #[allow(non_snake_case, unreachable_patterns, unreachable_code)]
    fn into_info(self) -> ::rocket::route::StaticInfo {
        fn monomorphized_function<'__r>(
            __req: &'__r ::rocket::request::Request<'_>,
            __data: ::rocket::data::Data<'__r>,
        ) -> ::rocket::route::BoxFuture<'__r> {
            ::std::boxed::Box::pin(async move {
                let __rocket_file: PathBuf =
                    match <PathBuf as ::rocket::request::FromSegments>::from_segments(
                        __req.routed_segments(0usize..),
                    ) {
                        ::std::result::Result::Ok(__v) => __v,
                        ::std::result::Result::Err(__error) => {
                            return {
                                {
                                    let lvl = ::log::Level::Warn;
                                    if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                                        ::log::__private_api_log(
                                            ::core::fmt::Arguments::new_v1(
                                                &[
                                                    "`",
                                                    ": ",
                                                    "` param guard parsed forwarding with error ",
                                                ],
                                                &match (&"file", &"PathBuf", &__error) {
                                                    (arg0, arg1, arg2) => [
                                                        ::core::fmt::ArgumentV1::new(
                                                            arg0,
                                                            ::core::fmt::Display::fmt,
                                                        ),
                                                        ::core::fmt::ArgumentV1::new(
                                                            arg1,
                                                            ::core::fmt::Display::fmt,
                                                        ),
                                                        ::core::fmt::ArgumentV1::new(
                                                            arg2,
                                                            ::core::fmt::Debug::fmt,
                                                        ),
                                                    ],
                                                },
                                            ),
                                            lvl,
                                            &("_", "blogger", "src/main.rs", 132u32),
                                        );
                                    }
                                };
                                ::rocket::outcome::Outcome::Forward(__data)
                            }
                        }
                    };
                let ___responder = static_files(__rocket_file).await;
                ::rocket::route::Outcome::from(__req, ___responder)
            })
        }
        ::rocket::route::StaticInfo {
            name: "static_files",
            method: ::rocket::http::Method::Get,
            uri: "/<file..>",
            handler: monomorphized_function,
            format: ::std::option::Option::None,
            rank: ::std::option::Option::None,
            sentinels: <[_]>::into_vec(box [
                {
                    #[allow(unused_imports)]
                    use ::rocket::sentinel::resolution::{Resolve, DefaultSentinel as _};
                    ::rocket::sentinel::Sentry {
                        type_id: std::any::TypeId::of::<PathBuf>(),
                        type_name: std::any::type_name::<PathBuf>(),
                        parent: None,
                        location: ("src/main.rs", 133u32, 29u32),
                        specialized: Resolve::<PathBuf>::SPECIALIZED,
                        abort: Resolve::<PathBuf>::abort,
                    }
                },
                {
                    #[allow(unused_imports)]
                    use ::rocket::sentinel::resolution::{Resolve, DefaultSentinel as _};
                    ::rocket::sentinel::Sentry {
                        type_id: std::any::TypeId::of::<Option<NamedFile>>(),
                        type_name: std::any::type_name::<Option<NamedFile>>(),
                        parent: None,
                        location: ("src/main.rs", 133u32, 41u32),
                        specialized: Resolve::<Option<NamedFile>>::SPECIALIZED,
                        abort: Resolve::<Option<NamedFile>>::abort,
                    }
                },
                {
                    #[allow(unused_imports)]
                    use ::rocket::sentinel::resolution::{Resolve, DefaultSentinel as _};
                    ::rocket::sentinel::Sentry {
                        type_id: std::any::TypeId::of::<NamedFile>(),
                        type_name: std::any::type_name::<NamedFile>(),
                        parent: None.or(Some(std::any::TypeId::of::<Option<NamedFile>>())),
                        location: ("src/main.rs", 133u32, 48u32),
                        specialized: Resolve::<NamedFile>::SPECIALIZED,
                        abort: Resolve::<NamedFile>::abort,
                    }
                },
            ]),
        }
    }
    #[doc(hidden)]
    pub fn into_route(self) -> ::rocket::Route {
        self.into_info().into()
    }
}
#[doc(hidden)]
pub use rocket_uri_macro_static_files_8287771120021529462 as rocket_uri_macro_static_files;
#[allow(dead_code)]
fn rocket() -> ::rocket::Rocket<::rocket::Build> {
    rocket::build()
        .mount("/", {
            let ___vec: ::std::vec::Vec<::rocket::Route> = <[_]>::into_vec(box [
                {
                    let ___struct = index {};
                    let ___item: ::rocket::Route = ___struct.into_route();
                    ___item
                },
                {
                    let ___struct = static_files {};
                    let ___item: ::rocket::Route = ___struct.into_route();
                    ___item
                },
                {
                    let ___struct = single_post {};
                    let ___item: ::rocket::Route = ___struct.into_route();
                    ___item
                },
            ]);
            ___vec
        })
        .attach(BlogDB::fairing())
        .attach(Template::fairing())
}
fn main() {
    ::rocket::async_main(async move {
        let _res = {
            let ___rocket: ::rocket::Rocket<::rocket::Build> = {
                rocket::build()
                    .mount("/", {
                        let ___vec: ::std::vec::Vec<::rocket::Route> = <[_]>::into_vec(box [
                            {
                                let ___struct = index {};
                                let ___item: ::rocket::Route = ___struct.into_route();
                                ___item
                            },
                            {
                                let ___struct = static_files {};
                                let ___item: ::rocket::Route = ___struct.into_route();
                                ___item
                            },
                            {
                                let ___struct = single_post {};
                                let ___item: ::rocket::Route = ___struct.into_route();
                                ___item
                            },
                        ]);
                        ___vec
                    })
                    .attach(BlogDB::fairing())
                    .attach(Template::fairing())
            };
            let ___rocket: ::rocket::Rocket<::rocket::Build> = ___rocket;
            ___rocket
        }
        .launch()
        .await;
    })
}
