use crate::websocket::ServerSocket;

use rustimate_core::ResponseMessage;

use actix::Addr;

#[derive(Debug)]
pub struct SendResponseMessage {
  msg: ResponseMessage
}

impl SendResponseMessage {
  pub(crate) fn msg(&self) -> &ResponseMessage {
    &self.msg
  }
}

impl actix::Message for SendResponseMessage {
  type Result = ();
}

#[derive(derive_more::Constructor)]
pub(crate) struct ServerSender {
  addr: Addr<ServerSocket>
}

impl rustimate_service::conn::SendCallback for ServerSender {
  fn send_message(&self, msg: ResponseMessage) {
    self.addr.do_send(SendResponseMessage { msg });
  }
}
