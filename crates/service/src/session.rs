use crate::files::FileService;

use rustimate_core::session::EstimateSession;
use rustimate_core::Result;

use std::sync::Arc;

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
    self.files.read_json(&format!("session/{}.json", key))
  }

  pub fn write_session(&mut self, session: &EstimateSession) -> Result<()> {
    self.files.create_dir_if_needed("session")?;
    slog::debug!(&self.log, "Writing session [{}]", session.key());
    self.files.write_json(session, &format!("session/{}.json", session.key()))
  }
}
