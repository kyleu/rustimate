use crate::ctx::ClientContext;

use maud::{html, Markup};
use rustimate_core::member::Member;
use rustimate_core::poll::Poll;
use uuid::Uuid;
use crate::poll_result::ResultSummary;

pub(crate) fn polls(ctx: &ClientContext, ps: Vec<&Poll>) -> Markup {
  html! {
    @if ps.is_empty() {
      li { "No polls" }
    } else {
      @for p in ps {
        (poll_summary(ctx, p))
      }
    }
  }
}

pub(crate) fn poll_summary(ctx: &ClientContext, p: &Poll) -> Markup {
  html! {
    li {
      a.(ctx.user_profile().link_class()) onclick=(crate::html::onclick_event("poll-detail", &p.id().to_string(), "")) {
        (p.title())
      }
    }
  }
}

pub(crate) fn vote_status(members: Vec<&Member>, votes: Vec<(Uuid, String)>) -> Markup {
  html! {
    div {
      @for member in members {
        div.vote-member {
          @if votes.iter().any(|x| &x.0 == member.user_id()) {
            div {
              (member.name())
              div {
                div.icon data-uk-icon="icon: check; ratio: 2" {}
              }
            }
          } @else {
            div {
              (member.name())
              div {
                div.icon data-uk-icon="icon: minus; ratio: 2" {}
              }
            }
          }
        }
      }
    }
    div.clear {}
  }
}

pub(crate) fn vote_choices(choices: &Vec<String>, current: Option<String>) -> Markup {
  html! {
    div {
      @for choice in choices {
        @if current.contains(choice) {
          div.vote-choice.active { (choice) }
        } @else {
          div.vote-choice onclick=(crate::html::onclick_event("select-choice", choice, "")) { (choice) }
        }
      }
    }
    div.clear {}
  }
}

pub(crate) fn vote_results(members: &[&Member], votes: &[(Uuid, String)]) -> Markup {
  html! {
    div {
      @for member in members {
        div.vote-member {
          div {
            (member.name())
            div {
              div.large-text {
                (votes.iter().find_map(|v| if &v.0 == member.user_id() { Some(v.1.clone()) } else { None }).unwrap_or("---".into()))
              }
            }
          }
        }
      }
    }
    div.clear {}
  }
}

pub(crate) fn vote_stats(summary: ResultSummary) -> Markup {
  html! {
    div {
      div {
        (format!("[{}/{}] votes counted", summary.valid_votes().len(), summary.valid_votes().len() + summary.invalid_votes().len()))
      }
      div {
        "Mean: "
        (summary.mean())
      }
      div {
        "Median: "
        (summary.median())
      }
      div {
        "Mode: "
        (summary.mode())
      }
    }
    div.clear {}
  }
}
