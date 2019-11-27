use crate::ctx::ClientContext;

use anyhow::Result;
use maud::html;
use rustimate_core::member::Member;
use rustimate_core::poll::Poll;
use rustimate_core::profile::UserProfile;
use rustimate_core::session_ctx::SessionContext;
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

      ResponseMessage::PollUpdate { poll } => on_update_poll(ctx, poll),
      ResponseMessage::MemberUpdate { member } => on_update_member(ctx, member),

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

  let mut svc = ctx.write().unwrap();

  let mut members: Vec<&Member> = session.members().iter().map(|x| x.1).collect();
  members.sort_by(|x, y| x.name().partial_cmp(&y.name()).unwrap());
  svc.replace_template(
    "member-listing",
    crate::templates::member::members(&svc, members, session.connected())
  )?;

  let mut polls: Vec<&Poll> = session.polls().iter().map(|x| x.1).collect();
  polls.sort_by(|x, y| x.idx().partial_cmp(&y.idx()).unwrap());
  svc.replace_template(
    "poll-listing",
    crate::templates::poll::polls(&svc, polls)
  )?;

  svc.on_session_joined(session);

  Ok(())
}

fn on_update_poll(ctx: &RwLock<ClientContext>, poll: Poll) -> Result<()> {
  Ok(())
}

fn on_update_member(ctx: &RwLock<ClientContext>, member: Member) -> Result<()> {
  Ok(())
}
