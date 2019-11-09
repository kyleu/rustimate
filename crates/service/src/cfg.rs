use crate::files::FileService;
use crate::session::SessionService;

use slog;
use std::sync::Arc;

/// Contains information about the running application
#[derive(Clone, Debug)]
pub struct AppConfig {
  task: String,
  address: String,
  port: u16,
  files: Arc<FileService>,
  sessions: Arc<std::sync::RwLock<SessionService>>,
  root_logger: slog::Logger,
  verbose: bool
}

impl AppConfig {
  pub fn new(task: String, address: String, port: u16, cfg_dir: String, root_logger: slog::Logger, verbose: bool) -> AppConfig {
    let files = Arc::new(FileService::new(&cfg_dir, &root_logger));
    let sessions = Arc::new(std::sync::RwLock::new(SessionService::new(Arc::clone(&files), root_logger.clone())));
    AppConfig {
      task,
      address,
      port,
      files: Arc::clone(&files),
      sessions,
      root_logger,
      verbose
    }
  }

  pub fn task(&self) -> &String {
    &self.task
  }

  pub fn address(&self) -> &String {
    &self.address
  }

  pub fn port(&self) -> u16 {
    self.port
  }

  pub fn files(&self) -> &FileService {
    &self.files
  }

  pub fn sessions(&self) -> &std::sync::RwLock<SessionService> {
    &self.sessions
  }

  pub fn root_logger(&self) -> &slog::Logger {
    &self.root_logger
  }

  pub fn verbose(&self) -> bool {
    self.verbose
  }
}
