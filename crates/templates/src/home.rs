use maud::{html, Markup};

use rustimate_core::Result;
use rustimate_service::{RequestContext, Router};

pub fn index(ctx: &RequestContext, router: &dyn Router) -> Result<Markup> {
  let content = html! {
    div.uk-text-center {
      h1.uk-heading-hero {
        "Welcome to rustimate"
      }
    }
    (session_form(ctx, router)?)
    (testbed_list(ctx, router)?)
  };
  crate::section(ctx, router, "Home", content)
}

fn session_form(ctx: &RequestContext, router: &dyn Router) -> Result<Markup> {
  Ok(html! {
    div {
      form action=(router.route_simple("session.create")?) method="get" {
        (crate::card(&ctx, html! {
          h4 { "Create Session" }
          input.uk-input type="text" placeholder="Title" name="key" {}
          button.uk-button.uk-button-default type="submit" { "Create Session" }
        }))
      }
    }
    div {
      form action=(router.route_simple("session.join_link")?) method="get" {
        (crate::card(&ctx, html! {
          h4 { "Join Session" }
          input.uk-input type="text" placeholder="Title" name="key" {}
          button.uk-button.uk-button-default type="submit" { "Join Session" }
        }))
      }
    }
  })
}

fn testbed_list(ctx: &RequestContext, router: &dyn Router) -> Result<Markup> {
  let ts = vec![
    ("dump", "Dump", "Displays a bunch of info about the app"),
    ("gallery", "Gallery", "Tests front-end components"),
    ("prototype", "Prototype", "A sandbox to play around in"),
    ("scroll", "Scroll", "Scrolling test"),
    ("crash", "Crash", "Simulates a server error"),
  ];
  Ok(crate::card(&ctx, html! {
    h4 { "Testbeds" }
    table.uk-table.uk-table-divider {
      tbody {
        @for t in ts {
          tr {
            td { a.(ctx.user_profile().link_class()) href=(router.route("testbed.detail", &[t.0])?) { (t.1) } }
            td { (t.2) }
          }
        }
      }
    }
  }))
}
