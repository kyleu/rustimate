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
#![warn(unsafe_code)]
#![warn(unused_extern_crates)]
#![warn(unused_import_braces)]
#![warn(unused_qualifications)]
#![warn(unused_results)]
#![warn(variant_size_differences)]
#![warn(missing_docs)]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/kyleu/rustimate/master/crates/assets/embed/favicon.ico")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/kyleu/rustimate/master/crates/assets/embed/favicon.png")]
#![doc(issue_tracker_base_url = "https://github.com/kyleu/rustimate/issues/")]
#![windows_subsystem = "windows"]

//! `rustimate` is a work in progress. Docs soon.
//! - [rustimate-assets](rustimate_assets)
//! - [rustimate-client](rustimate_client)
//! - [rustimate-controllers](rustimate_controllers)
//! - [rustimate-core](rustimate_core)
//! - [rustimate-service](rustimate_service)
//! - [rustimate-templates](rustimate_templates)

mod args;
mod cfg;
mod log;
mod server;

#[cfg(test)]
pub mod tests;

/// Application entrypoint, creates and starts the server
pub fn go() -> rustimate_core::Result<()> {
  let cfg = crate::cfg::cfg_from_args();
  let (port_tx, _) = std::sync::mpsc::channel();
  crate::server::start_server(cfg, port_tx)
}

/// External app entrypoint, calls `go()` directly and swallows errors
#[no_mangle]
pub extern "C" fn libgo() {
  match go() {
    Ok(_) => println!("Successfully started [{}]", rustimate_core::APPNAME),
    Err(e) => println!("Error starting [{}]: {}", rustimate_core::APPNAME, e)
  };
}

#[cfg(target_os = "android")]
#[allow(non_snake_case)]
pub mod android {
  extern crate jni;

  use self::jni::objects::JClass;
  use self::jni::JNIEnv;
  use super::go;

  #[no_mangle]
  #[allow(unsafe_code)]
  pub unsafe extern "C" fn Java_com_kyleu_rustimate_rustimate_go(env: JNIEnv<'_>, _: JClass<'_>) {
    println!("Android!");
    go();
  }
}
