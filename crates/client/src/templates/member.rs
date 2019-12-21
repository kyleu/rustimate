use crate::ctx::ClientContext;

use maud::{html, Markup};
use rustimate_core::member::Member;
use std::collections::HashSet;
use uuid::Uuid;

pub(crate) fn members(ctx: &ClientContext, ms: Vec<&Member>, conn: &HashSet<Uuid>) -> Markup {
  html! {
    @for m in ms {
      (member_summary(ctx, m, conn.contains(m.user_id())))
    }
  }
}

pub(crate) fn member_summary(ctx: &ClientContext, m: &Member, conn: bool) -> Markup {
  html! {
    li {
      a.(ctx.user_profile().link_class()) onclick=(crate::html::onclick_event("member-detail", &m.user_id().to_string(), "")) {
        (m.name())
        (conn_el(conn))
      }
    }
  }
}

pub(crate) fn member_detail(_ctx: &ClientContext, m: &Member, conn: bool) -> Markup {
  html! {
    em { (format!("{}", m.role())) }
    div {
      (conn_el(conn))
    }
  }
}

fn conn_el(conn: bool) -> Markup {
  html! {
    @if conn {
      span { " (Connected)" }
    } else {
      span { " (Not Connected)" }
    }
  }
}
