#![forbid(unsafe_code)]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/kyleu/rustimate/master/crates/assets/embed/favicon.ico")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/kyleu/rustimate/master/crates/assets/embed/favicon.png")]
#![doc(issue_tracker_base_url = "https://github.com/kyleu/rustimate/issues/")]

//! `rustimate-assets` contains embedded static files intended to be served from the web application.

use rust_embed::RustEmbed;

/// Embeds assets from `./embed`, providing static files for the web app
#[derive(RustEmbed)]
#[folder = "$CARGO_MANIFEST_DIR/embed"]
pub struct Asset;
