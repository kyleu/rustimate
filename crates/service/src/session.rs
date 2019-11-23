use crate::files::FileService;

use rustimate_core::member::Member;
use rustimate_core::poll::{Poll, Vote};
use rustimate_core::session::EstimateSession;

use anyhow::Result;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct SessionService {
  files: Arc<FileService>,
  log: slog::Logger
}

impl SessionService {
  pub fn new(files: Arc<FileService>, logger: slog::Logger) -> SessionService {
    let log = logger.new(slog::o!("service" => "session"));
    SessionService { files, log }
  }

  pub fn read_session(&self, key: &str) -> Result<EstimateSession> {
    slog::debug!(&self.log, "Loading session [{}]", key);
    let p = format!("session/{}/session.json", key);
    if self.files.exists(&p) {
      self.files.read_json(&p)
    } else {
      Err(anyhow::anyhow!("No session found with key [{}]", key))
    }
  }

  pub fn write_session(&mut self, session: &EstimateSession) -> Result<()> {
    self.files.create_dir_if_needed(&format!("session/{}", session.key()))?;
    slog::debug!(&self.log, "Writing session [{}]", session.key());
    self.files.write_json(session, &format!("session/{}/session.json", session.key()))
  }

  pub fn read_members(&self, key: &str) -> Result<Vec<Member>> {
    slog::debug!(&self.log, "Loading members for session [{}]", key);
    let p = format!("session/{}/members.json", key);
    if self.files.exists(&p) {
      self.files.read_json(&p)
    } else {
      Ok(vec![])
    }
  }

  pub fn add_member(&mut self, key: &str, m: Member) -> Result<()> {
    let mut current = self.read_members(key)?;
    if current.iter().any(|x| x.user_id() == m.user_id()) {
      Ok(())
    } else {
      current.push(m);
      self.write_members(key, current)
    }
  }

  pub fn write_members(&mut self, key: &str, vm: Vec<Member>) -> Result<()> {
    let p = format!("session/{}/members.json", key);
    self.files.write_json(vm, &p)
  }

  pub fn read_polls(&self, key: &str) -> Result<Vec<Poll>> {
    slog::debug!(&self.log, "Loading polls for session [{}]", key);
    let p = format!("session/{}/polls.json", key);
    if self.files.exists(&p) {
      self.files.read_json(&p)
    } else {
      Ok(vec![])
    }
  }

  pub fn update_poll(&mut self, session_key: &str, poll_id: Uuid, title: String, author_id: Uuid) -> Result<Poll> {
    let mut current = self.read_polls(session_key)?;
    match current.iter_mut().find(|x| x.id() == &poll_id) {
      Some(p) => {
        p.set_title(title);
        Ok(p.clone())
      }
      None => {
        let p = Poll::new(poll_id, session_key.to_string(), current.len() as u32, author_id, title);
        current.push(p.clone());
        self.write_polls(session_key, current)?;
        Ok(p)
      }
    }
  }

  pub fn write_polls(&mut self, key: &str, vm: Vec<Poll>) -> Result<()> {
    let p = format!("session/{}/polls.json", key);
    self.files.write_json(vm, &p)
  }

  pub fn read_votes(&self, key: &str) -> Result<Vec<Vote>> {
    slog::debug!(&self.log, "Loading votes for session [{}]", key);
    let p = format!("session/{}/votes.json", key);
    if self.files.exists(&p) {
      self.files.read_json(&p)
    } else {
      Ok(vec![])
    }
  }

  pub fn add_vote(&mut self, key: &str, v: Vote) -> Result<()> {
    let mut current = self.read_votes(key)?;
    if current.iter().any(|x| x.poll_id() == v.poll_id() && x.user_id() == v.user_id()) {
      Ok(())
    } else {
      current.push(v);
      self.write_votes(key, current)
    }
  }

  pub fn write_votes(&mut self, key: &str, vm: Vec<Vote>) -> Result<()> {
    let p = format!("session/{}/votes.json", key);
    self.files.write_json(vm, &p)
  }
}
