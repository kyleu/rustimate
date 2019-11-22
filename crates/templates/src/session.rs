use crate::utils::onclick_event;

use anyhow::Result;
use maud::{html, Markup};
use rustimate_core::session::EstimateSession;
use rustimate_service::{RequestContext, Router};

pub fn detail(ctx: &RequestContext, router: &dyn Router, es: &EstimateSession) -> Result<Markup> {
  let content = html! {
    div.uk-container {
      div.uk-margin-top.uk-grid-small uk-grid? {
        div."uk-width-1-3" {
          (crate::card(&ctx, html! {
            h3 { (ctx.user_profile().name()) }
          }))
        }
        div."uk-width-2-3" {
          (crate::card(&ctx, html! {
            h3 { (es.title()) }
          }))
        }
        div."uk-width-1-3" {
          (crate::card(&ctx, html! {
            h4 { "Other Members" }
            ul {
              li { "No other members" }
            }
            a.(ctx.user_profile().link_class()) uk-toggle? href="#add-member-modal" { "Invite Members" }
          }))
        }
        div."uk-width-2-3" {
          (crate::card(&ctx, html! {
            h4 { "Estimates, Polls, Stories, Whatever" }
            ul {
              li { "No polls" }
            }
            a.(ctx.user_profile().link_class()) uk-toggle? onclick="document.getElementById('poll-modal-input').value = '';" href="#add-poll-modal" { "Add Poll" }
          }))
        }
        div."uk-width-1-1" {
          (socket(ctx)?)
        }
      }
    }
    div#add-member-modal uk-modal? {
      div.uk-modal-dialog.uk-modal-body {
        h2.uk-modal-title { "Add Member" }
        "I'm working on it, for now just copy/paste the url"
        p.uk-text-right {
          button.uk-button.uk-button-default.uk-modal-close type="button" { "Cancel" }
        }
      }
    }
    div#add-poll-modal uk-modal? {
      div.uk-modal-dialog.uk-modal-body {
        h2.uk-modal-title { "Add Poll" }
        input.uk-input#poll-modal-input type="text" placeholder="Poll Question" name="poll" {}
        p.uk-text-right {
          button.uk-button.uk-button-default.uk-modal-close type="button" { "Cancel" }
          button.uk-button.uk-button-primary.uk-modal-close type="button" onclick=(onclick_event("add-poll", "", "document.getElementById('poll-modal-input').value")) { "Add Poll" }
        }
      }
    }
    script src=(router.route_static("client.js")?) defer? {}
  };
  Ok(html! {
    (crate::simple(ctx, router, "Home", content)?)
  })
}

fn socket(ctx: &RequestContext) -> Result<Markup> {
  Ok(crate::card(&ctx, html! {
    h4 { "WebSocket" }
    div {
      a.(ctx.user_profile().link_class()) href="" onclick=(onclick_event("send-ping", "", "")) { "Send Ping" }
    }
    div#socket-status { "Connecting..." }
    div#socket-results.uk-margin-top { }
  }))
}
