#![feature(backtrace)]
#![feature(proc_macro_hygiene)]
#![feature(exclusive_range_pattern)]
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
#![warn(clippy::cargo_common_metadata)]
#![warn(clippy::cast_lossless)]
#![warn(clippy::clone_on_ref_ptr)]
#![warn(clippy::copy_iterator)]
#![warn(clippy::default_trait_access)]
#![warn(clippy::else_if_without_else)]
#![warn(clippy::empty_line_after_outer_attr)]
#![warn(clippy::exit)]
#![warn(clippy::expl_impl_clone_on_copy)]
#![warn(clippy::explicit_into_iter_loop)]
#![warn(clippy::explicit_iter_loop)]
#![warn(clippy::fallible_impl_from)]
#![warn(clippy::filter_map)]
#![warn(clippy::filter_map_next)]
#![warn(clippy::find_map)]
#![warn(clippy::get_unwrap)]
#![warn(clippy::if_not_else)]
#![warn(clippy::invalid_upcast_comparisons)]
#![warn(clippy::items_after_statements)]
#![warn(clippy::large_digit_groups)]
#![warn(clippy::large_stack_arrays)]
#![warn(clippy::linkedlist)]
#![warn(clippy::map_flatten)]
#![warn(clippy::match_same_arms)]
#![warn(clippy::maybe_infinite_iter)]
#![warn(clippy::mem_forget)]
#![warn(clippy::missing_const_for_fn)]
#![warn(clippy::multiple_inherent_impl)]
#![warn(clippy::mut_mut)]
#![warn(clippy::mutex_integer)]
#![warn(clippy::needless_borrow)]
#![warn(clippy::needless_continue)]
#![warn(clippy::needless_pass_by_value)]
#![warn(clippy::option_map_unwrap_or)]
#![warn(clippy::option_map_unwrap_or_else)]
#![warn(clippy::option_unwrap_used)]
#![warn(clippy::panic)]
#![warn(clippy::path_buf_push_overwrite)]
#![warn(clippy::print_stdout)]
#![warn(clippy::pub_enum_variant_names)]
#![warn(clippy::redundant_closure_for_method_calls)]
#![warn(clippy::replace_consts)]
#![warn(clippy::result_map_unwrap_or_else)]
#![warn(clippy::result_unwrap_used)]
#![warn(clippy::same_functions_in_if_condition)]
#![warn(clippy::similar_names)]
#![warn(clippy::todo)]
#![warn(clippy::too_many_lines)]
#![warn(clippy::type_repetition_in_bounds)]
#![warn(clippy::unimplemented)]
#![warn(clippy::unreachable)]
#![warn(clippy::unused_self)]
#![warn(clippy::use_debug)]
#![warn(clippy::use_self)]
#![warn(clippy::used_underscore_binding)]
#![warn(clippy::wildcard_dependencies)]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/kyleu/rustimate/master/crates/assets/embed/favicon.ico")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/kyleu/rustimate/master/crates/assets/embed/favicon.png")]
#![doc(issue_tracker_base_url = "https://github.com/kyleu/rustimate/issues/")]

//! `rustimate-templates` contains Maud templates used by the server to render responses

pub mod admin;
pub mod connections;
pub(crate) mod components {
  pub(crate) mod backtrace;
  pub(crate) mod card;
  pub(crate) mod colors;
  pub(crate) mod flash;
  pub(crate) mod header;
  pub(crate) mod navbar;
  pub(crate) mod page;
  pub(crate) mod script;
  pub(crate) mod simple;
}
pub mod error;
pub mod gallery;
pub mod home;
pub mod profile;
pub mod session;
pub mod settings;
pub mod testbed;
pub(crate) mod utils;

pub(crate) use components::card::card;
pub(crate) use components::simple::{section, simple};
