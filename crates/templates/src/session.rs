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
            h3.left style="margin: 0;" { (ctx.user_profile().name()) }
            div.right {
              a.(ctx.user_profile().link_class()) href="" onclick=(onclick_event("profile-detail", "", "")) title="Change Name" {
                span data-uk-icon="icon: cog" {}
              }
            }
          }))
        }
        div."uk-width-2-3" {
          (crate::card(&ctx, html! {
            h3.left style="margin: 0;" { (es.title()) }
            div.right {
              a.(ctx.user_profile().link_class()) href="" onclick=(onclick_event("session-detail", "", "")) title="Edit Title" {
                span data-uk-icon="icon: cog" {}
              }
            }
          }))
        }
        div."uk-width-1-3" {
          (crate::card(&ctx, html! {
            h4 { "Other Members" }
            ul#member-listing {
              li { "Loading members..." }
            }
            a.(ctx.user_profile().link_class()) uk-toggle? href="#add-member-modal" { "Invite Members" }
          }))
        }
        div."uk-width-2-3" {
          (crate::card(&ctx, html! {
            h4 { "Estimates, Polls, Stories, Whatever" }
            ul#poll-listing {
              li { "Loading polls..." }
            }
            a.(ctx.user_profile().link_class()) uk-toggle? onclick="document.getElementById('poll-modal-input').value = '';" href="#add-poll-modal" { "Add Poll" }
            " "
            a.(ctx.user_profile().link_class()) onclick=(onclick_event("update-poll", "", "'xxxxxxx'")) href="" { "Test" }
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
          button.uk-button.uk-button-primary.uk-modal-close type="button" onclick=(onclick_event("update-poll", "", "document.getElementById('poll-modal-input').value")) { "Add Poll" }
        }
      }
    }
    div#member-detail-modal uk-modal? {
      div.uk-modal-dialog.uk-modal-body {
        h2.uk-modal-title#member-detail-name { "..." }
        div#member-detail-content { "..." }
        p.uk-text-right {
          button.uk-button.uk-button-default.uk-modal-close type="button" { "Close" }
        }
      }
    }
    div#poll-detail-modal uk-modal? {
      div.uk-modal-dialog.uk-modal-body {
        h2.uk-modal-title#poll-detail-title { "..." }
        div#poll-detail-content { "..." }
        p.uk-text-right {
          button.uk-button.uk-button-default.uk-modal-close type="button" { "Close" }
        }
      }
    }
    div#profile-detail-modal uk-modal? {
      div.uk-modal-dialog.uk-modal-body {
        h2.uk-modal-title { "Edit Profile" }
        input.uk-input#profile-detail-modal-input type="text" placeholder="Name" name="name" {}
        p.uk-text-right {
          button.uk-button.uk-button-default.uk-modal-close type="button" { "Cancel" }
          button.uk-button.uk-button-primary.uk-modal-close type="button" onclick=(onclick_event("update-profile", "", "document.getElementById('profile-detail-modal-input').value")) { "Change Name" }
        }
      }
    }
    div#session-detail-modal uk-modal? {
      div.uk-modal-dialog.uk-modal-body {
        h2.uk-modal-title { "Edit Session" }
        input.uk-input#session-detail-modal-input type="text" placeholder="Name" name="name" {}
        p.uk-text-right {
          button.uk-button.uk-button-default.uk-modal-close type="button" { "Cancel" }
          button.uk-button.uk-button-primary.uk-modal-close type="button" onclick=(onclick_event("update-session", "", "document.getElementById('session-detail-modal-input').value")) { "Change Name" }
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
