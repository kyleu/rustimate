use crate::ctx::ClientContext;

use anyhow::Result;
use maud::html;
use rustimate_core::poll::{Poll, PollStatus};
use rustimate_core::RequestMessage;
use rustimate_core::util::NotificationLevel;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::RwLock;
use uuid::Uuid;

pub(crate) fn on_update_poll(ctx: &RwLock<ClientContext>, poll: Poll) -> Result<()> {
  {
    let mut svc = ctx.write().expect("Cannot lock ClientContext for write");
    if let Some(ref mut x) = svc.session_ctx_mut() {
      x.set_poll(poll);
    }
  }
  {
    let svc = ctx.read().expect("Cannot lock ClientContext for read");

    if let Some(session) = svc.session_ctx() {
      render_polls(&svc, session.polls())?;
    }
  }
  Ok(())
}

pub(crate) fn render_polls(svc: &ClientContext, polls: &HashMap<Uuid, Poll>) -> Result<()> {
  let mut polls: Vec<&Poll> = polls.iter().map(|x| x.1).collect();
  match get_active_poll_id(svc) {
    Ok(active) => {
      match polls.iter().find(|x| x.id() == &active) {
        Some(p) => poll_status_dom(svc, p.status().clone()),
        None => Ok(())
      }
    }
    Err(_) => Ok(())
  }?;
  polls.sort_by(|x, y| x.idx().partial_cmp(&y.idx()).expect("No index?"));
  svc.replace_template("poll-listing", crate::templates::poll::polls(svc, polls))
}

pub(crate) fn get_active_poll_id(ctx: &ClientContext) -> Result<Uuid> {
  Uuid::from_str(&ctx.get_input_value("active-poll-id")?).map_err(|_| anyhow::anyhow!("Unable to find active poll id"))
}

pub(crate) fn on_poll_detail(ctx: &ClientContext, id: Uuid) -> Result<()> {
  if let Some(sc) = ctx.session_ctx() {
    if let Some(p) = sc.polls().get(&id) {
      ctx.replace_template("poll-detail-title", html!((p.title())))?;
      ctx.set_input_value("active-poll-id", &format!("{}", id))?;
      poll_status_dom(ctx, p.status().clone())?;
    }
  }
  crate::js::show_modal("poll-detail-modal");
  Ok(())
}

pub(crate) fn on_set_poll_status(ctx: &ClientContext, status: PollStatus) -> Result<()> {
  ctx.send(&RequestMessage::SetPollStatus { poll: get_active_poll_id(ctx)?, status })?;
  Ok(())
}

pub(crate) fn on_poll_submit(ctx: &ClientContext, v: &str) -> Result<()> {
  if v.is_empty() {
    crate::logging::notify(&NotificationLevel::Warn, "Enter a question next time")
  } else {
    ctx.send(&RequestMessage::SetPollTitle {
      id: Uuid::new_v4(),
      title: v.into()
    })
  }
}

fn poll_status_dom(ctx: &ClientContext, status: PollStatus) -> Result<()> {
  ctx.set_visible("poll-section-pending", status == PollStatus::Pending)?;
  ctx.set_visible("poll-section-active", status == PollStatus::Active)?;
  ctx.set_visible("poll-section-complete", status == PollStatus::Complete)?;

  if status == PollStatus::Active {
    crate::votes::render_votes(ctx)?;
  }
  if status == PollStatus::Complete {
    crate::votes::render_results(ctx)?;
  }

  Ok(())
}
