use crate::RequestContext;

use rustimate_core::{RequestMessage, ResponseMessage, Result};

/// Core application logic, routing [RequestMessage](rustimate_core::RequestMessage)s and emitting [ResponseMessage](rustimate_core::ResponseMessage)s.
#[derive(Debug)]
pub struct MessageHandler {
  id: uuid::Uuid,
  session_id: String,
  ctx: RequestContext,
  log: slog::Logger
}

impl MessageHandler {
  pub fn new(id: uuid::Uuid, session_id: String, ctx: RequestContext) -> MessageHandler {
    let log = ctx
      .log()
      .new(slog::o!("service" => "message_handler", "session" => session_id.clone()));
    MessageHandler { id, session_id, ctx, log }
  }

  pub fn id(&self) -> &uuid::Uuid {
    &self.id
  }

  pub fn session_id(&self) -> &String {
    &self.session_id
  }

  pub fn ctx(&self) -> &RequestContext {
    &self.ctx
  }

  pub fn on_open(&self) -> Result<Vec<ResponseMessage>> {
    let hello = ResponseMessage::Hello {
      session_id: *self.id(),
      u: Box::new((*self.ctx.user_profile()).clone()),
      b: !self.ctx.app().verbose()
    };
    let join_session = {
      let svc = self.ctx.app().sessions().read().unwrap();

      ResponseMessage::SessionJoined {
        session: Box::new(svc.read_session(&self.session_id)?),
        members: svc.read_members(&self.session_id)?,
        connected: vec!(),
        polls: svc.read_polls(&self.session_id)?,
        votes: svc.read_votes(&self.session_id)?
      }
    };
    Ok(vec![hello, join_session])
  }

  pub fn on_closed(&self) -> Vec<ResponseMessage> {
    Vec::new()
  }

  pub fn on_message(&self, msg: RequestMessage) -> Result<Vec<ResponseMessage>> {
    let mut ret = Vec::new();
    match msg {
      RequestMessage::Ping { v } => ret.push(ResponseMessage::Pong { v }),
      msg => slog::warn!(self.log, "Unhandled RequestMessage [{:?}]", msg)
    }
    Ok(ret)
  }

  pub fn on_error(&self) {}

  pub fn log(&self) -> &slog::Logger {
    &self.log
  }
}
