use crate::conn::ConnectionCache;
use crate::files::FileService;
use crate::session::SessionService;

use std::sync::Arc;

/// Contains information about the running application
#[derive(Clone, Debug)]
pub struct AppConfig {
  task: String,
  address: String,
  port: u16,
  files: Arc<FileService>,
  connections: Arc<ConnectionCache>,
  session_svc: Arc<SessionService>,
  root_logger: slog::Logger,
  verbose: bool
}

impl AppConfig {
  pub fn new(task: String, address: String, port: u16, cfg_dir: &str, root_logger: slog::Logger, verbose: bool) -> Self {
    let files = FileService::new(cfg_dir, &root_logger);
    Self {
      task,
      address,
      port,
      files: Arc::new(files.clone()),
      connections: Arc::new(ConnectionCache::new(&root_logger)),
      session_svc: Arc::new(SessionService::new(files, &root_logger)),
      root_logger,
      verbose
    }
  }

  pub const fn task(&self) -> &String {
    &self.task
  }

  pub const fn address(&self) -> &String {
    &self.address
  }

  pub const fn port(&self) -> u16 {
    self.port
  }

  pub fn files(&self) -> &FileService {
    &self.files
  }

  pub fn connections(&self) -> &ConnectionCache {
    &self.connections
  }

  pub fn session_svc(&self) -> &SessionService {
    &self.session_svc
  }

  pub fn root_logger(&self) -> &slog::Logger {
    &self.root_logger
  }

  pub const fn verbose(&self) -> bool {
    self.verbose
  }
}
