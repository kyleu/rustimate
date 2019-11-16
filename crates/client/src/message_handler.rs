use crate::ctx::ClientContext;

use rustimate_core::session_ctx::SessionContext;
use rustimate_core::profile::UserProfile;
use rustimate_core::{ResponseMessage, Result};

use maud::html;
use std::sync::RwLock;

pub(crate) struct MessageHandler {}

impl MessageHandler {
  pub(crate) fn handle(ctx: &RwLock<ClientContext>, msg: ResponseMessage) -> Result<()> {
    debug!("Message received: {:?}", msg);
    match msg {
      ResponseMessage::Connected { connection_id, u, b } => on_connected(ctx, connection_id, &u, b)?,
      ResponseMessage::Pong { v } => on_pong(ctx, v)?,
      ResponseMessage::SessionJoined { session, members, connected, polls, votes } => {
        on_session_joined(ctx, SessionContext::new(session, members, connected, polls, votes))?
      }
      _ => warn!("Unhandled ResponseMessage [{:?}]", msg)
    };
    Ok(())
  }
}

fn on_connected(ctx: &RwLock<ClientContext>, session_id: uuid::Uuid, u: &UserProfile, b: bool) -> Result<()> {
  ctx.write().unwrap().on_connected(session_id, u.clone(), b);
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
  ctx.write().unwrap().on_session_joined(session);
  Ok(())
}

