#![feature(option_expect_none)]
#![feature(fn_traits)]
#![feature(unboxed_closures)]
#![warn(anonymous_parameters)]
#![warn(bare_trait_objects)]
#![warn(elided_lifetimes_in_paths)]
#![warn(missing_debug_implementations)]
#![warn(single_use_lifetimes)]
#![warn(trivial_casts)]
#![warn(unreachable_pub)]
#![forbid(unsafe_code)]
#![warn(unused_extern_crates)]
#![warn(unused_import_braces)]
#![warn(unused_qualifications)]
#![warn(unused_results)]
#![warn(variant_size_differences)]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/kyleu/rustimate/master/crates/assets/embed/favicon.ico")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/kyleu/rustimate/master/crates/assets/embed/favicon.png")]
#![doc(issue_tracker_base_url = "https://github.com/kyleu/rustimate/issues/")]

//! `rustimate-core` contains definitions that are shared between server and client.

pub const APPNAME: &str = "rustimate";

/// Static information about the environent and build invocation
pub mod build_info;
pub mod member;
pub mod messages {
  pub mod req;
  pub mod rsp;
}
pub mod poll;
pub mod profile;
pub mod session;
pub mod util;

#[doc(inline)]
pub use crate::messages::req::RequestMessage;
#[doc(inline)]
pub use crate::messages::rsp::ResponseMessage;

#[cfg(test)]
pub mod tests;
