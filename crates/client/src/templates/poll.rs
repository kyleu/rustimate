use crate::ctx::ClientContext;

use maud::{html, Markup};
use rustimate_core::poll::Poll;

pub(crate) fn poll(ctx: &ClientContext, p: &Poll) -> Markup {
  html!(
    li {
      a.(ctx.user_profile().link_class()) onclick=(crate::html::onclick_event("poll-detail", &p.id().to_string(), "")) {
        (p.title())
      }
    }
  )
}

pub(crate) fn polls(ctx: &ClientContext, ps: Vec<&Poll>) -> Markup {
  html!(
    @if ps.is_empty() {
      li { "No polls" }
    } else {
      @for p in ps {
        (poll(ctx, p))
      }
    }
  )
}
