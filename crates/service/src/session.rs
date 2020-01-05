use crate::files::FileService;

use rustimate_core::member::{Member, MemberRole};
use rustimate_core::poll::{Poll, PollStatus, Vote};
use rustimate_core::session::EstimateSession;

use anyhow::Result;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct SessionService {
  files: Arc<RwLock<FileService>>,
  log: slog::Logger
}

impl SessionService {
  pub fn new(files: FileService, logger: &slog::Logger) -> Self {
    let log = logger.new(slog::o!("service" => "session"));
    Self {
      files: Arc::new(RwLock::new(files)),
      log
    }
  }

  pub fn read_session(&self, key: &str) -> Result<EstimateSession> {
    slog::debug!(&self.log, "Loading session [{}]", key);
    let p = format!("session/{}/session.json", key);
    let f = self.files.read().expect("Cannot lock FileService for read");
    if f.exists(&p) {
      f.read_json(&p)
    } else {
      Err(anyhow::anyhow!("No session found with key [{}]", key))
    }
  }

  pub fn write_session(&self, session: &EstimateSession) -> Result<()> {
    slog::debug!(&self.log, "Writing session [{}]", session.key());
    let x = self.files.write().expect("Cannot lock FileService for read");
    x.create_dir_if_needed(&format!("session/{}", session.key()))?;
    x.write_json(session, &format!("session/{}/session.json", session.key()))
  }

  pub fn read_members(&self, key: &str) -> Result<Vec<Member>> {
    slog::debug!(&self.log, "Loading members for session [{}]", key);
    let p = format!("session/{}/members.json", key);
    let f = self.files.read().expect("Cannot lock FileService for read");
    if f.exists(&p) {
      f.read_json(&p)
    } else {
      Ok(vec![])
    }
  }

  pub fn update_member(&self, session_key: &str, user_id: Uuid, name: String, role: Option<MemberRole>) -> Result<Member> {
    let mut current = self.read_members(session_key)?;
    let ret = match current.iter_mut().find(|x| x.user_id() == &user_id) {
      Some(m) => {
        m.set_name(name.clone());
        let _ = role.map(|r| m.set_role(r));
        Ok(m.clone())
      }
      None => {
        let m = Member::new(user_id, name.clone(), role.unwrap_or(MemberRole::Participant));
        current.push(m.clone());
        Ok(m)
      }
    };
    {
      let f = self.files.write().expect("Cannot lock FileService for write");
      let mut profile = crate::profile::load(&f, user_id);
      profile.set_name(&name);
      crate::profile::save(&f, &user_id, &profile)?;
    }
    self.write_members(session_key, current)?;
    ret
  }

  pub fn write_members(&self, key: &str, vm: Vec<Member>) -> Result<()> {
    let p = format!("session/{}/members.json", key);
    self.files.write().expect("Cannot lock FileService for write").write_json(vm, &p)
  }

  pub fn read_polls(&self, key: &str) -> Result<Vec<Poll>> {
    slog::debug!(&self.log, "Loading polls for session [{}]", key);
    let p = format!("session/{}/polls.json", key);
    let f = self.files.read().expect("Cannot lock FileService for read");
    if f.exists(&p) {
      f.read_json(&p)
    } else {
      Ok(vec![])
    }
  }

  pub fn update_poll(
    &self, session_key: &str, poll_id: Uuid, title: Option<String>, status: Option<PollStatus>, author_id: Uuid
  ) -> Result<Poll> {
    let mut current = self.read_polls(session_key)?;
    let ret = match current.iter_mut().find(|x| x.id() == &poll_id) {
      Some(p) => {
        p.set_title(title.unwrap_or_else(|| p.title().into()));
        p.set_status(status.unwrap_or_else(|| p.status().clone()));
        Ok(p.clone())
      }
      None => {
        let p = Poll::new(
          poll_id,
          current.len() as u32,
          author_id,
          title.unwrap_or_else(|| "New poll".into()),
          status.unwrap_or(PollStatus::Pending)
        );
        current.push(p.clone());
        Ok(p)
      }
    };
    self.write_polls(session_key, current)?;
    ret
  }

  pub fn write_polls(&self, key: &str, vm: Vec<Poll>) -> Result<()> {
    let p = format!("session/{}/polls.json", key);
    self.files.write().expect("Cannot lock FileService for read").write_json(vm, &p)
  }

  pub fn read_votes(&self, key: &str) -> Result<Vec<Vote>> {
    slog::debug!(&self.log, "Loading votes for session [{}]", key);
    let p = format!("session/{}/votes.json", key);
    let f = self.files.read().expect("Cannot lock FileService for read");
    if f.exists(&p) {
      f.read_json(&p)
    } else {
      Ok(vec![])
    }
  }

  pub fn update_vote(&self, key: &str, vote: Vote) -> Result<Vote> {
    let mut current = self.read_votes(key)?;
    let ret = match current
      .iter_mut()
      .find(|x| x.poll_id() == vote.poll_id() && x.user_id() == vote.user_id())
    {
      Some(v) => {
        v.set_choice(vote.choice().into());
        Ok(v.clone())
      }
      None => {
        current.push(vote.clone());
        Ok(vote)
      }
    };
    self.write_votes(key, current)?;
    ret
  }

  pub fn write_votes(&self, key: &str, vm: Vec<Vote>) -> Result<()> {
    let p = format!("session/{}/votes.json", key);
    self.files.write().expect("Cannot lock FileService for read").write_json(vm, &p)
  }
}
