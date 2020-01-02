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
          (crate::card(ctx, &html! {
            h3.left.nomargin#profile-name-label { (ctx.user_profile().name()) }
            div.right {
              a.(ctx.user_profile().link_class()) href="" onclick=(onclick_event("profile-detail", "", "")) title="Change Name" {
                span data-uk-icon="icon: cog" {}
              }
            }
          }))
        }
        div."uk-width-2-3" {
          (crate::card(ctx, &html! {
            h3.left.nomargin#session-name-label { (es.title()) }
            div.right {
              a.(ctx.user_profile().link_class()) href="" onclick=(onclick_event("session-detail", "", "")) title="Edit Title" {
                span data-uk-icon="icon: cog" {}
              }
            }
          }))
        }
        div."uk-width-1-3" {
          (crate::card(ctx, &html! {
            h4 { "Other Members" }
            ul#member-listing {
              li { "Loading members..." }
            }
            a.(ctx.user_profile().link_class()) uk-toggle? href="#add-member-modal" { "Invite Members" }
          }))
        }
        div."uk-width-2-3" {
          (crate::card(ctx, &html! {
            h4 { "Estimates, Polls, Stories, Whatever" }
            ul#poll-listing {
              li { "Loading polls..." }
            }
            a.(ctx.user_profile().link_class()) uk-toggle? onclick="document.getElementById('poll-modal-input').value = '';" href="#add-poll-modal" { "Add Poll" }
          }))
        }
        div."uk-width-1-1" {
          (socket(ctx)?)
        }
      }
    }
    (crate::modals::modals())
    script src=(router.route_static("client.js")?) defer? {}
  };
  Ok(html! {
    (crate::simple(ctx, router, "Home", &content)?)
  })
}

fn socket(ctx: &RequestContext) -> Result<Markup> {
  Ok(crate::card(ctx, &html! {
    h4 { "WebSocket" }
    div {
      a.(ctx.user_profile().link_class()) href="" onclick=(onclick_event("send-ping", "", "")) { "Send Ping" }
    }
    div#socket-status { "Connecting..." }
    div#socket-results.uk-margin-top { }
  }))
}
