use crate::ctx::ClientContext;
use rustimate_core::{RequestMessage, Result};

pub(crate) struct EventHandler {}

impl EventHandler {
  pub(crate) fn handle(ctx: &ClientContext, t: &str, k: &str, v: &str) -> Result<()> {
    match t {
      "send-ping" => ctx.send(RequestMessage::Ping {
        v: js_sys::Date::now() as i64
      }),
      "add-member" => info!("Add Member!"),
      "add-poll" => info!("Add Poll!"),
      _ => warn!("Unhandled event [{}] with [k:{}], [v:{}]", t, k, v)
    }
    Ok(())
  }
}
