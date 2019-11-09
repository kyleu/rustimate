use crate::files::FileService;

use rustimate_core::member::Member;
use rustimate_core::session::EstimateSession;
use rustimate_core::{Error, Result};

use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct SessionService {
  files: Arc<FileService>,
  log: slog::Logger,
  sockets: std::collections::HashMap<String, String>
}

impl SessionService {
  pub fn new(files: Arc<FileService>, logger: slog::Logger) -> SessionService {
    let log = logger.new(slog::o!("service" => "session"));
    let sockets = std::collections::HashMap::new();
    SessionService { files, log, sockets }
  }

  pub fn read_session(&self, key: &str) -> Result<EstimateSession> {
    slog::debug!(&self.log, "Loading session [{}]", key);
    let p = format!("session/{}/session.json", key);
    if self.files.exists(&p) {
      self.files.read_json(&p)
    } else {
      Err(Error::from(format!("No session found with key [{}]", key)))
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
    if current.iter().find(|x| x.user_id() == m.user_id()).is_some() {
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
}
