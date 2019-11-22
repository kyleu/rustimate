use crate::RequestContext;

use anyhow::Result;
use rustimate_core::{RequestMessage, ResponseMessage};

/// Core application logic, routing [RequestMessage](rustimate_core::RequestMessage)s and emitting [ResponseMessage](rustimate_core::ResponseMessage)s.
#[derive(Debug)]
pub struct MessageHandler {
  connection_id: uuid::Uuid,
  channel_id: String,
  ctx: RequestContext,
  log: slog::Logger
}

impl MessageHandler {
  pub fn new(connection_id: uuid::Uuid, channel_id: String, ctx: RequestContext) -> MessageHandler {
    let log = ctx
      .log()
      .new(slog::o!("connection" => format!("{}", connection_id), "service" => "message_handler", "channel" => channel_id.clone()));
    MessageHandler {
      connection_id,
      channel_id,
      ctx,
      log
    }
  }

  pub fn connection_id(&self) -> &uuid::Uuid {
    &self.connection_id
  }

  pub fn channel_id(&self) -> &String {
    &self.channel_id
  }

  pub fn ctx(&self) -> &RequestContext {
    &self.ctx
  }

  pub fn on_open(&self) -> Result<Vec<ResponseMessage>> {
    let connected = ResponseMessage::Connected {
      connection_id: *self.connection_id(),
      u: Box::new((*self.ctx.user_profile()).clone()),
      b: !self.ctx.app().verbose()
    };
    let join_session = {
      let svc = self.ctx.app().session_svc().read().unwrap();

      ResponseMessage::SessionJoined {
        session: Box::new(svc.read_session(&self.channel_id)?),
        members: svc.read_members(&self.channel_id)?,
        connected: vec![],
        polls: svc.read_polls(&self.channel_id)?,
        votes: svc.read_votes(&self.channel_id)?
      }
    };
    Ok(vec![connected, join_session])
  }

  pub fn on_closed(&self) -> Vec<ResponseMessage> {
    Vec::new()
  }

  pub fn on_message(&self, msg: RequestMessage) -> Result<Vec<ResponseMessage>> {
    let mut ret = Vec::new();
    match msg {
      RequestMessage::Ping { v } => ret.push(ResponseMessage::Pong { v }),
      RequestMessage::UpdatePoll { id, title } => self.on_update_poll(id, title),
      msg => slog::warn!(self.log, "Unhandled RequestMessage [{:?}]", msg)
    }
    Ok(ret)
  }

  pub fn on_error(&self) {}

  pub fn on_update_poll(&self, id: uuid::Uuid, title: String) {
    slog::info!(self.log(), "Adding poll [{}: {}] for session [{}]", id, title, self.channel_id())
  }

  pub fn log(&self) -> &slog::Logger {
    &self.log
  }
}
