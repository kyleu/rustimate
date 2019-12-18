use crate::ctx::ClientContext;
use crate::session_ctx::SessionContext;

use anyhow::Result;
use maud::html;
use rustimate_core::profile::UserProfile;
use rustimate_core::session::EstimateSession;
use rustimate_core::ResponseMessage;
use std::sync::RwLock;
use uuid::Uuid;

pub(crate) struct MessageHandler {}

impl MessageHandler {
  pub(crate) fn handle(ctx: &RwLock<ClientContext>, msg: ResponseMessage) -> Result<()> {
    debug!("Message received: {:?}", msg);
    match msg {
      ResponseMessage::Connected {
        connection_id,
        user_id,
        u,
        b
      } => on_connected(ctx, connection_id, user_id, &u, b),
      ResponseMessage::Pong { v } => on_pong(ctx, v),
      ResponseMessage::Notification { level, content } => crate::logging::notify(level, &content),

      // Custom Messages
      ResponseMessage::SessionJoined {
        session,
        members,
        connected,
        polls,
        votes
      } => on_session_joined(ctx, SessionContext::new(*session, members, connected, polls, votes)),

      ResponseMessage::UpdateSession { session } => on_update_session(ctx, session),
      ResponseMessage::UpdatePoll { poll } => crate::polls::on_update_poll(ctx, poll),
      ResponseMessage::UpdateMember { member } => crate::members::on_update_member(ctx, member),

      _ => {
        warn!("Unhandled ResponseMessage [{:?}]", msg);
        Ok(())
      }
    }
  }
}

fn on_connected(ctx: &RwLock<ClientContext>, connection_id: Uuid, user_id: Uuid, u: &UserProfile, b: bool) -> Result<()> {
  ctx.write().unwrap().on_connected(connection_id, user_id, u.clone(), b);
  let c = ctx.read().unwrap();
  let _ = c.append_template(
    "socket-results",
    "div",
    html!((format!("Connect message received, {} connection", if b { "binary" } else { "text" })))
  )?;
  Ok(())
}

fn on_pong(ctx: &RwLock<ClientContext>, v: i64) -> Result<()> {
  let now = js_sys::Date::now() as i64;
  let msg = format!("Pong: [{}ms] elapsed", now - v);
  let _ = ctx.read().unwrap().append_template("socket-results", "div", html!((msg)))?;
  info!("{}", msg);
  Ok(())
}

fn on_session_joined(ctx: &RwLock<ClientContext>, session: SessionContext) -> Result<()> {
  info!("Session details received for [{}]", session.session().key());
  {
    let svc = ctx.read().unwrap();
    crate::members::render_members(&svc, &session.members(), session.connected())?;
    crate::polls::render_polls(&svc, &session.polls())?;
  }
  {
    let mut svc = ctx.write().unwrap();
    svc.on_session_joined(session);
  }
  Ok(())
}

fn on_update_session(ctx: &RwLock<ClientContext>, session: EstimateSession) -> Result<()> {
  {
    let svc = ctx.read().unwrap();
    svc.replace_template("session-name-label", html!((session.title())))?;
  }
  {
    let mut svc = ctx.write().unwrap();
    if let Some(ref mut x) = svc.session_ctx_mut() {
      x.set_session(session);
    }
  }
  Ok(())
}
