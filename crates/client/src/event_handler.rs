use crate::ctx::ClientContext;

use anyhow::Result;
use rustimate_core::util::NotificationLevel;
use rustimate_core::RequestMessage;
use uuid::Uuid;

pub(crate) struct EventHandler {}

impl EventHandler {
  pub(crate) fn handle(ctx: &ClientContext, t: &str, k: &str, v: &str) -> Result<()> {
    match t {
      "send-ping" => ctx.send(RequestMessage::Ping {
        v: js_sys::Date::now() as i64
      }),
      "update-poll" if v.is_empty() => crate::logging::notify(NotificationLevel::Warn, "Enter a question next time"),
      "update-poll" => ctx.send(RequestMessage::UpdatePoll {
        id: Uuid::new_v4(),
        title: v.into()
      }),
      "member-detail" => on_member_detail(),
      "poll-detail" => on_poll_detail(),
      _ => {
        warn!("Unhandled event [{}] with [k:{}], [v:{}]", t, k, v);
        Ok(())
      }
    }
  }
}

fn on_member_detail() -> Result<()> {
  crate::js::show_modal("member-detail-modal");
  Ok(())
}

fn on_poll_detail() -> Result<()> {
  crate::js::show_modal("poll-detail-modal");
  Ok(())
}
