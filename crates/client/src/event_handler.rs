use crate::ctx::ClientContext;

use anyhow::Result;
use maud::html;
use rustimate_core::util::NotificationLevel;
use rustimate_core::RequestMessage;
use uuid::Uuid;

pub(crate) struct EventHandler {}

impl EventHandler {
  pub(crate) fn handle(ctx: &ClientContext, t: &str, k: &str, v: &str) -> Result<()> {
    match t {
      "update-profile" => on_update_profile(ctx, v),
      "update-session" => on_update_session(ctx, v),
      "member-detail" => on_member_detail(ctx, Uuid::parse_str(k).unwrap()),
      "poll-detail" => on_poll_detail(ctx, Uuid::parse_str(k).unwrap()),
      "profile-detail" => on_profile_detail(ctx),
      "send-ping" => ctx.send(RequestMessage::Ping { v: js_sys::Date::now() as i64 }),
      "session-detail" => on_session_detail(ctx),
      "update-poll" => on_update_poll(ctx, v),
      _ => {
        warn!("Unhandled event [{}] with [k:{}], [v:{}]", t, k, v);
        Ok(())
      }
    }
  }
}

fn on_member_detail(ctx: &ClientContext, id: Uuid) -> Result<()> {
  if let Some(sc) = ctx.session_ctx() {
    if let Some(m) = sc.members().get(&id) {
      ctx.replace_template("member-detail-name", html!((m.name())))?;
      ctx.replace_template("member-detail-content", crate::templates::member::member_detail(ctx, m, false))?;
    }
  }
  crate::js::show_modal("member-detail-modal");
  Ok(())
}

fn on_poll_detail(ctx: &ClientContext, id: Uuid) -> Result<()> {
  if let Some(sc) = ctx.session_ctx() {
    if let Some(p) = sc.polls().get(&id) {
      ctx.replace_template("poll-detail-title", html!((p.title())))?;
      ctx.replace_template("poll-detail-content", crate::templates::poll::poll_detail(ctx, p))?;
    }
  }
  crate::js::show_modal("poll-detail-modal");
  Ok(())
}

fn on_profile_detail(ctx: &ClientContext) -> Result<()> {
  ctx.set_input_value("profile-detail-modal-input", ctx.user_profile().name())?;
  crate::js::show_modal("profile-detail-modal");
  Ok(())
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
    crate::logging::notify(NotificationLevel::Warn, "Enter a name next time")
  } else {
    ctx.send(RequestMessage::UpdateSelf {
      name: name.into()
    })
  }
}

fn on_update_session(ctx: &ClientContext, name: &str) -> Result<()> {
  if name.is_empty() {
    crate::logging::notify(NotificationLevel::Warn, "Enter a session name next time")
  } else {
    if let Some(sc) = ctx.session_ctx() {
      let choices: Vec<String> = sc.session().choices().iter().cloned().collect();
      ctx.send(RequestMessage::UpdateSession {
        name: name.into(),
        choices
      })
    } else {
      Ok(())
    }
  }
}

fn on_update_poll(ctx: &ClientContext, v: &str) -> Result<()> {
  if v.is_empty() {
    crate::logging::notify(NotificationLevel::Warn, "Enter a question next time")
  } else {
    ctx.send(RequestMessage::UpdatePoll {
      id: Uuid::new_v4(),
      title: v.into()
    })
  }
}
