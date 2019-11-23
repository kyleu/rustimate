use crate::ctx::ClientContext;

use anyhow::Result;
use maud::html;
use rustimate_core::profile::UserProfile;
use rustimate_core::session_ctx::SessionContext;
use rustimate_core::util::NotificationLevel;
use rustimate_core::ResponseMessage;
use std::sync::RwLock;

pub(crate) struct MessageHandler {}

impl MessageHandler {
  pub(crate) fn handle(ctx: &RwLock<ClientContext>, msg: ResponseMessage) -> Result<()> {
    debug!("Message received: {:?}", msg);
    match msg {
      ResponseMessage::Connected { connection_id, u, b } => on_connected(ctx, connection_id, &u, b)?,
      ResponseMessage::Pong { v } => on_pong(ctx, v)?,
      ResponseMessage::Notification { level, content } => on_notification(level, content)?,

      // Custom Messages
      ResponseMessage::SessionJoined {
        session,
        members,
        connected,
        polls,
        votes
      } => on_session_joined(ctx, SessionContext::new(*session, members, connected, polls, votes))?,
      _ => warn!("Unhandled ResponseMessage [{:?}]", msg)
    };
    Ok(())
  }
}

fn on_connected(ctx: &RwLock<ClientContext>, connection_id: uuid::Uuid, u: &UserProfile, b: bool) -> Result<()> {
  ctx.write().unwrap().on_connected(connection_id, u.clone(), b);
  let c = ctx.read().unwrap();
  let _ = c.append_template(
    "socket-results",
    "div",
    html!((format!("Connect message received, {} connection", if b { "binary" } else { "text" })))
  )?;
  Ok(())
}

fn on_notification(level: NotificationLevel, content: String) -> Result<()> {
  crate::logging::notify(level, &content);
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

  let mut svc = ctx.write().unwrap();
  svc.replace_template(
    "member-listing",
    crate::templates::member::members(&svc, session.members().iter().map(|x| x.1).collect(), session.connected())
  )?;
  svc.replace_template(
    "poll-listing",
    crate::templates::poll::polls(&svc, session.polls().iter().map(|x| x.1).collect())
  )?;
  svc.on_session_joined(session);

  Ok(())
}
