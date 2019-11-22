use rustimate_service::Router;

use actix_web::HttpRequest;
use anyhow::Result;

#[derive(derive_more::Constructor, Debug)]
pub(crate) struct RequestRouter {
  req: HttpRequest
}

impl Router for RequestRouter {
  fn route(&self, path: &str, args: &[&str]) -> Result<String> {
    self
      .req
      .url_for(path, args)
      .map(|x| x.path().into())
      .map_err(|_| anyhow::anyhow!("Unable to find route for [{}]", path))
  }
}
