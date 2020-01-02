use crate::RequestContext;

use anyhow::Result;
use rustimate_core::member::MemberRole;
use rustimate_core::poll::{PollStatus, Vote};
use rustimate_core::{RequestMessage, ResponseMessage};
use uuid::Uuid;

/// Core application logic, routing [RequestMessage](rustimate_core::RequestMessage)s and emitting [ResponseMessage](rustimate_core::ResponseMessage)s.
#[derive(Debug)]
pub struct MessageHandler {
  connection_id: Uuid,
  channel_id: String,
  ctx: RequestContext,
  log: slog::Logger
}

impl MessageHandler {
  pub fn new(connection_id: Uuid, channel_id: String, ctx: RequestContext) -> Self {
    let o = slog::o!("connection" => format!("{}", connection_id), "service" => "message_handler", "channel" => channel_id.clone());
    let log = ctx.log().new(o);
    Self {
      connection_id,
      channel_id,
      ctx,
      log
    }
  }

  pub const fn connection_id(&self) -> &Uuid {
    &self.connection_id
  }

  pub const fn channel_id(&self) -> &String {
    &self.channel_id
  }

  pub const fn ctx(&self) -> &RequestContext {
    &self.ctx
  }

  pub fn log(&self) -> &slog::Logger {
    &self.log
  }

  pub fn on_open(&self) -> Result<()> {
    self.send_to_self(ResponseMessage::Connected {
      connection_id: *self.connection_id(),
      user_id: *self.ctx().user_id(),
      u: Box::new((*self.ctx.user_profile()).clone()),
      b: !self.ctx.app().verbose()
    })?;

    let svc = self.ctx.app().session_svc();

    let role = if svc.read_members(&self.channel_id)?.is_empty() {
      MemberRole::Creator
    } else {
      MemberRole::Participant
    };
    let member = svc.update_member(
      &self.channel_id,
      *self.ctx.user_id(),
      self.ctx.user_profile().name().clone(),
      Some(role)
    )?;
    self.send_to_channel_except_self(&ResponseMessage::UpdateMember { member })?;

    self.send_to_self(ResponseMessage::SessionJoined {
      session: Box::new(svc.read_session(&self.channel_id)?),
      members: svc.read_members(&self.channel_id)?,
      connected: vec![],
      polls: svc.read_polls(&self.channel_id)?,
      votes: svc.read_votes(&self.channel_id)?
    })?;

    Ok(())
  }

  pub fn on_closed(&self) -> Result<()> {
    slog::debug!(self.log, "Closing connection for [{}:{}]", self.connection_id, self.channel_id);
    Ok(())
  }

  pub fn on_message(&self, msg: RequestMessage) -> Result<()> {
    match msg {
      RequestMessage::Ping { v } => self.send_to_self(ResponseMessage::Pong { v }),
      RequestMessage::SetPollTitle { id, title } => self.set_poll_title(id, &title),
      RequestMessage::SetPollStatus { poll, status } => self.set_poll_status(poll, status),
      RequestMessage::SubmitVote { poll, vote } => self.on_submit_vote(poll, vote),
      RequestMessage::UpdateSelf { name } => self.on_update_self(&name),
      RequestMessage::UpdateSession { name, choices } => self.on_update_session(name, choices),
      msg => {
        slog::warn!(self.log, "Unhandled RequestMessage [{:?}]", msg);
        Ok(())
      }
    }
  }

  fn set_poll_title(&self, id: Uuid, title: &str) -> Result<()> {
    let svc = self.ctx().app().session_svc();
    let poll = svc.update_poll(self.channel_id(), id, Some(title.into()), None, *self.ctx().user_id())?;
    self.send_to_channel(&ResponseMessage::UpdatePoll { poll })?;
    slog::info!(self.log(), "Updated poll [{}: {}] for session [{}]", id, title, self.channel_id());
    Ok(())
  }

  fn set_poll_status(&self, id: Uuid, status: PollStatus) -> Result<()> {
    let svc = self.ctx().app().session_svc();
    let poll = svc.update_poll(self.channel_id(), id, None, Some(status), *self.ctx().user_id())?;
    self.send_to_channel(&ResponseMessage::UpdatePoll { poll })?;
    slog::info!(self.log(), "Updated poll [{}] for session [{}]", id, self.channel_id());
    Ok(())
  }

  fn on_submit_vote(&self, poll: Uuid, vote: String) -> Result<()> {
    let svc = self.ctx().app().session_svc();
    let vote = Vote::new(poll, *self.ctx().user_id(), vote);
    let vote = svc.update_vote(self.channel_id(), vote)?;
    self.send_to_channel(&ResponseMessage::UpdateVote { vote })
  }

  fn on_update_self(&self, name: &str) -> Result<()> {
    let svc = self.ctx().app().session_svc();
    let member = svc.update_member(self.channel_id(), *self.ctx().user_id(), name.into(), None)?;
    self.send_to_channel(&ResponseMessage::UpdateMember { member })?;
    Ok(())
  }

  fn on_update_session(&self, title: String, choices: Vec<String>) -> Result<()> {
    let svc = self.ctx().app().session_svc();
    let mut session = svc.read_session(self.channel_id())?;
    session.set_title(title);
    session.set_choices(choices);
    svc.write_session(&session)?;
    self.send_to_channel(&ResponseMessage::UpdateSession { session })?;
    Ok(())
  }

  fn send_to_self(&self, msg: ResponseMessage) -> Result<()> {
    self.ctx().app().connections().send_connection(self.connection_id(), msg);
    Ok(())
  }

  fn send_to_channel(&self, msg: &ResponseMessage) -> Result<()> {
    self.ctx().app().connections().send_channel(self.channel_id(), msg);
    Ok(())
  }

  fn send_to_channel_except_self(&self, msg: &ResponseMessage) -> Result<()> {
    self
      .ctx()
      .app()
      .connections()
      .send_channel_except(self.channel_id(), &[self.connection_id()], msg);
    Ok(())
  }
}
