#![warn(anonymous_parameters)]
#![warn(bare_trait_objects)]
#![warn(elided_lifetimes_in_paths)]
#![warn(missing_debug_implementations)]
#![warn(single_use_lifetimes)]
#![warn(trivial_casts)]
#![warn(unreachable_pub)]
#![warn(unsafe_code)]
#![warn(unused_extern_crates)]
#![warn(unused_import_braces)]
#![warn(unused_qualifications)]
#![warn(unused_results)]
#![warn(variant_size_differences)]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/kyleu/rustimate/master/crates/assets/embed/favicon.ico")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/kyleu/rustimate/master/crates/assets/embed/favicon.png")]
#![doc(issue_tracker_base_url = "https://github.com/kyleu/rustimate/issues/")]

//! `rustimate-service` contains the primary logic for the application. It receives [RequestMessage](rustimate_core::RequestMessage)s and emits [ResponseMessage](rustimate_core::ResponseMessage)s

pub mod cfg;
pub mod conn;
pub mod ctx;
pub mod files;
pub mod handler;
pub mod profile;
pub mod session;

#[doc(inline)]
pub use cfg::AppConfig;
#[doc(inline)]
pub use ctx::RequestContext;
#[doc(inline)]
pub use ctx::Router;
