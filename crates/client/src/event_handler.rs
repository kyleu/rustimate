use crate::ctx::ClientContext;

use anyhow::Result;
use rustimate_core::RequestMessage;

pub(crate) struct EventHandler {}

impl EventHandler {
  pub(crate) fn handle(ctx: &ClientContext, t: &str, k: &str, v: &str) -> Result<()> {
    match t {
      "send-ping" => ctx.send(RequestMessage::Ping {
        v: js_sys::Date::now() as i64
      }),
      "update-poll" if v.is_empty() => crate::js::notify("warn", "Enter a question next time"),
      "update-poll" => ctx.send(RequestMessage::UpdatePoll {
        id: uuid::Uuid::new_v4(),
        title: v.into()
      }),
      _ => warn!("Unhandled event [{}] with [k:{}], [v:{}]", t, k, v)
    }
    Ok(())
  }
}
