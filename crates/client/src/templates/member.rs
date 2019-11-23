use crate::ctx::ClientContext;

use maud::{html, Markup};
use rustimate_core::member::Member;
use std::collections::HashSet;

pub(crate) fn member(ctx: &ClientContext, m: &Member, conn: bool) -> Markup {
  html!(
    li {
      a.(ctx.user_profile().link_class()) href="" {
        @if conn {
          span { "[Connected]" }
        } else {
          span { "[Not Connected]" }
        }
        (m.name())
      }
    }
  )
}

pub(crate) fn members(ctx: &ClientContext, ms: Vec<&Member>, conn: &HashSet<uuid::Uuid>) -> Markup {
  html!(
    @if ms.len() < 2 {
      li { "No other members" }
    } else {
      @for m in ms {
        (member(ctx, m, conn.contains(m.user_id())))
      }
    }
  )
}
