use crate::ctx::ClientContext;

use anyhow::Result;
use rustimate_core::poll::Poll;
use std::collections::HashMap;
use std::sync::RwLock;
use uuid::Uuid;

pub(crate) fn on_update_poll(ctx: &RwLock<ClientContext>, poll: Poll) -> Result<()> {
  {
    let mut svc = ctx.write().unwrap();
    if let Some(ref mut x) = svc.session_ctx_mut() {
      x.set_poll(poll);
    }
  }
  {
    let svc = ctx.read().unwrap();

    if let Some(session) = svc.session_ctx() {
      render_polls(&svc, &session.polls())?;
    }
  }
  Ok(())
}

pub(crate) fn render_polls(svc: &ClientContext, polls: &HashMap<Uuid, Poll>) -> Result<()> {
  let mut polls: Vec<&Poll> = polls.iter().map(|x| x.1).collect();
  polls.sort_by(|x, y| x.idx().partial_cmp(&y.idx()).unwrap());
  svc.replace_template(
    "poll-listing",
    crate::templates::poll::polls(&svc, polls)
  )
}

