use maud::{html, Markup, PreEscaped};

use crate::components;
use rustimate_core::Result;
use rustimate_service::{RequestContext, Router};

pub(crate) fn page(ctx: &RequestContext, router: &dyn Router, title: &str, content: Markup) -> Result<Markup> {
  Ok(html! {
    (PreEscaped("<!DOCTYPE html>"))
    html lang="en" {
      (components::header::header(&ctx, router, &format!("{} - {}", title, rustimate_core::APPNAME))?)
      body.(ctx.user_profile().theme().body_class()) {
        (content)
      }
    }
  })
}
