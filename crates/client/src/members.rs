use crate::ctx::ClientContext;

use anyhow::Result;
use maud::html;
use rustimate_core::member::Member;
use std::collections::HashSet;
use std::sync::RwLock;
use uuid::Uuid;

pub(crate) fn on_update_member(ctx: &RwLock<ClientContext>, member: Member) -> Result<()> {
  {
    let mut svc = ctx.write().expect("Cannot lock ClientContext for write");
    if let Some(ref mut x) = svc.session_ctx_mut() {
      x.set_member(member);
    }
  }
  {
    let svc = ctx.read().expect("Cannot lock ClientContext for read");
    if let Some(session) = svc.session_ctx() {
      render_members(&svc, session.members_sorted(), session.connected())?;
    }
  }
  Ok(())
}

pub(crate) fn render_members(svc: &ClientContext, members: Vec<&Member>, connected: &HashSet<Uuid>) -> Result<()> {
  let me = &svc.user_id().expect("No current user!");
  let _ = members.iter().find_map(|m| {
    if m.user_id() == me {
      let _ = svc.replace_template("profile-name-label", html!((m.name())));
      let _ = svc.set_input_value("profile-detail-modal-input", m.name());
    }
    Some(m)
  });
  svc.replace_template("member-listing", crate::templates::member::members(svc, members, connected))
}

pub(crate) fn on_member_detail(ctx: &ClientContext, id: Uuid) -> Result<()> {
  if let Some(sc) = ctx.session_ctx() {
    if let Some(m) = sc.members().get(&id) {
      ctx.replace_template("member-detail-name", html!((m.name())))?;
      ctx.replace_template("member-detail-content", crate::templates::member::member_detail(ctx, m, false))?;
    }
  }
  crate::js::show_modal("member-detail-modal");
  Ok(())
}
