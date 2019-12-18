use crate::RequestContext;

use anyhow::Result;
use rustimate_core::member::MemberRole;
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
  pub fn new(connection_id: Uuid, channel_id: String, ctx: RequestContext) -> MessageHandler {
    let o = slog::o!("connection" => format!("{}", connection_id), "service" => "message_handler", "channel" => channel_id.clone());
    let log = ctx.log().new(o);
    MessageHandler {
      connection_id,
      channel_id,
      ctx,
      log
    }
  }

  pub fn connection_id(&self) -> &Uuid {
    &self.connection_id
  }

  pub fn channel_id(&self) -> &String {
    &self.channel_id
  }

  pub fn ctx(&self) -> &RequestContext {
    &self.ctx
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
    let member = svc.update_member(&self.channel_id, *self.ctx.user_id(), self.ctx.user_profile().name().clone(), role)?;
    self.send_to_channel_except_self(ResponseMessage::UpdateMember { member })?;

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
    Ok(())
  }

  pub fn on_message(&self, msg: RequestMessage) -> Result<()> {
    match msg {
      RequestMessage::Ping { v } => self.send_to_self(ResponseMessage::Pong { v }),
      RequestMessage::UpdatePoll { id, title } => self.on_update_poll(id, title),
      RequestMessage::UpdateSelf { name } => self.on_update_self(name),
      RequestMessage::UpdateSession { name, choices } => self.on_update_session(name, choices),
      msg => {
        slog::warn!(self.log, "Unhandled RequestMessage [{:?}]", msg);
        Ok(())
      }
    }
  }

  fn on_update_poll(&self, id: Uuid, title: String) -> Result<()> {
    let svc = self.ctx().app().session_svc();
    let poll = svc.update_poll(self.channel_id(), id, title.clone(), *self.ctx().user_id())?;
    self.send_to_channel(ResponseMessage::UpdatePoll { poll })?;
    slog::info!(self.log(), "Updated poll [{}: {}] for session [{}]", id, title, self.channel_id());
    Ok(())
  }

  fn on_update_self(&self, name: String) -> Result<()> {
    let svc = self.ctx().app().session_svc();
    let member = svc.update_member_name(self.channel_id(), *self.ctx().user_id(), name.clone())?;
    self.send_to_channel(ResponseMessage::UpdateMember { member })?;
    slog::info!(
      self.log(),
      "Updated member [{}: {}] for session [{}]",
      self.ctx().user_id(),
      name,
      self.channel_id()
    );
    Ok(())
  }

  fn on_update_session(&self, title: String, choices: Vec<String>) -> Result<()> {
    let svc = self.ctx().app().session_svc();
    let mut session = svc.read_session(self.channel_id())?;
    slog::info!(self.log(), "Updating session [{}: {:?}] for session [{}]", &title, &choices, self.channel_id());
    session.set_title(title);
    session.set_choices(choices);
    let _ = svc.write_session(&session)?;
    self.send_to_channel(ResponseMessage::UpdateSession { session })?;
    Ok(())
  }

  pub fn log(&self) -> &slog::Logger {
    &self.log
  }

  fn send_to_self(&self, msg: ResponseMessage) -> Result<()> {
    self.ctx().app().connections().send_connection(self.connection_id(), msg);
    Ok(())
  }

  fn send_to_channel(&self, msg: ResponseMessage) -> Result<()> {
    self.ctx().app().connections().send_channel(self.channel_id(), msg);
    Ok(())
  }

  fn send_to_channel_except_self(&self, msg: ResponseMessage) -> Result<()> {
    self
      .ctx()
      .app()
      .connections()
      .send_channel_except(self.channel_id(), vec![self.connection_id()], msg);
    Ok(())
  }
}
