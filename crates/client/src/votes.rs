use crate::ctx::ClientContext;
use crate::poll_result::ResultSummary;

use anyhow::Result;
use std::sync::RwLock;
use rustimate_core::poll::Vote;

pub(crate) fn on_update_vote(ctx: &RwLock<ClientContext>, v: Vote) -> Result<()> {
  {
    let mut svc = ctx.write().expect("Cannot lock ClientContext for write");
    if let Some(ref mut x) = svc.session_ctx_mut() {
      x.set_vote(v.poll_id().clone(), v.user_id().clone(), v.choice().clone());
    }
  }
  {
    let svc = ctx.read().expect("Cannot lock ClientContext for read");
    render_votes(&svc)?;
    render_results(&svc)?;
  }
  Ok(())
}

pub(crate) fn render_votes(svc: &ClientContext) -> Result<()> {
  match crate::polls::get_active_poll_id(svc) {
    Ok(active) => {
      if let Some(sc) = svc.session_ctx() {
        let votes = sc.votes_by_poll(&active);
        let me = &svc.user_id().expect("No current user!");
        let current = votes.iter().find_map(|v| if &v.0 == me { Some(v.1.clone()) } else { None });
        svc.replace_template("poll-vote-status", crate::templates::poll::vote_status(sc.members_sorted(), votes))?;
        svc.replace_template("poll-vote-choices", crate::templates::poll::vote_choices(sc.session().choices(), current))?;
      }
      Ok(())
    }
    Err(_) => Ok(())
  }
}

pub(crate) fn render_results(svc: &ClientContext) -> Result<()> {
  match crate::polls::get_active_poll_id(svc) {
    Ok(active) => {
      if let Some(sc) = svc.session_ctx() {
        let members = sc.members_sorted();
        let votes = sc.votes_by_poll(&active);
        svc.replace_template("poll-complete-summary", crate::templates::poll::vote_results(&members, &votes))?;
        let summary = ResultSummary::new(&members, &votes);
        svc.replace_template("poll-complete-stats", crate::templates::poll::vote_stats(summary))?;
      }
      Ok(())
    }
    Err(_) => Ok(())
  }
}
