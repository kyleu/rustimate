use crate::ctx::ClientContext;

use anyhow::Result;
use maud::html;
use rustimate_core::member::Member;
use std::collections::{HashMap, HashSet};
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
      render_members(&svc, session.members(), session.connected())?;
    }
  }
  Ok(())
}

pub(crate) fn render_members(svc: &ClientContext, members: &HashMap<Uuid, Member>, connected: &HashSet<Uuid>) -> Result<()> {
  let me = &svc.user_id().expect("No current user!");
  let mut members: Vec<&Member> = members
    .iter()
    .map(|x| {
      if x.1.user_id() == me {
        let _ = svc.replace_template("profile-name-label", html!((x.1.name())));
        let _ = svc.set_input_value("profile-detail-modal-input", x.1.name());
      }
      x.1
    })
    .collect();
  members.sort_by(|x, y| x.name().partial_cmp(y.name()).expect("Can't compare?"));
  svc.replace_template("member-listing", crate::templates::member::members(svc, members, connected))
}
