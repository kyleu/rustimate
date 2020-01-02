use crate::ctx::ClientContext;

use anyhow::Result;
use rustimate_core::util::NotificationLevel;
use rustimate_core::poll::PollStatus;
use rustimate_core::RequestMessage;
use std::str::FromStr;
use uuid::Uuid;
pub(crate) struct EventHandler {}

impl EventHandler {
  pub(crate) fn handle(ctx: &ClientContext, t: &str, k: &str, v: &str) -> Result<()> {
    match t {
      "update-profile" => on_update_profile(ctx, v),
      "update-session" => on_update_session(ctx, v),
      "member-detail" => crate::members::on_member_detail(ctx, Uuid::parse_str(k).expect("Invalid UUID")),
      "poll-detail" => crate::polls::on_poll_detail(ctx, Uuid::parse_str(k).expect("Invalid UUID")),
      "profile-detail" => on_profile_detail(ctx),
      "select-choice" => on_select_choice(ctx, k),
      "send-ping" => ctx.send(&RequestMessage::Ping {
        v: js_sys::Date::now() as i64
      }),
      "session-detail" => on_session_detail(ctx),
      "set-poll-status" => crate::polls::on_set_poll_status(ctx, PollStatus::from_str(k)?),
      "submit-poll" => crate::polls::on_poll_submit(ctx, v),
      _ => {
        warn!("Unhandled event [{}] with [k:{}], [v:{}]", t, k, v);
        Ok(())
      }
    }
  }
}

fn on_profile_detail(ctx: &ClientContext) -> Result<()> {
  ctx.set_input_value("profile-detail-modal-input", ctx.user_profile().name())?;
  crate::js::show_modal("profile-detail-modal");
  Ok(())
}

fn on_select_choice(ctx: &ClientContext, vote: &str) -> Result<()> {
  ctx.send(&RequestMessage::SubmitVote {
    poll: crate::polls::get_active_poll_id(ctx)?,
    vote: vote.to_string()
  })
}

fn on_session_detail(ctx: &ClientContext) -> Result<()> {
  if let Some(sc) = ctx.session_ctx() {
    ctx.set_input_value("session-detail-modal-input", sc.session().title())?;
  }
  crate::js::show_modal("session-detail-modal");
  Ok(())
}

fn on_update_profile(ctx: &ClientContext, name: &str) -> Result<()> {
  if name.is_empty() {
    crate::logging::notify(&NotificationLevel::Warn, "Enter a name next time")
  } else {
    ctx.send(&RequestMessage::UpdateSelf { name: name.into() })
  }
}

fn on_update_session(ctx: &ClientContext, name: &str) -> Result<()> {
  if name.is_empty() {
    crate::logging::notify(&NotificationLevel::Warn, "Enter a session name next time")
  } else if let Some(sc) = ctx.session_ctx() {
    let choices: Vec<String> = sc.session().choices().to_vec();
    ctx.send(&RequestMessage::UpdateSession {
      name: name.into(),
      choices
    })
  } else {
    Ok(())
  }
}
