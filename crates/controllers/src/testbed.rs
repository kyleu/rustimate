use actix_session::Session;
use actix_web::{web, HttpRequest, HttpResponse};

use rustimate_core::Error;
use rustimate_service::AppConfig;

/// Available at `/testbed/{key}`
pub fn testbed_key(session: Session, cfg: web::Data<AppConfig>, key: web::Path<String>, req: HttpRequest) -> HttpResponse {
  crate::act(&session, &cfg, &req, |ctx| {
    let k: &str = &key;
    match k {
      "dump" => rustimate_templates::testbed::dump(&ctx),
      "gallery" => rustimate_templates::testbed::gallery(&ctx),
      "prototype" => rustimate_templates::testbed::prototype(&ctx),
      "scroll" => rustimate_templates::testbed::scroll(&ctx),
      _ => Err(Error::from(format!("Cannot find testbed matching [{}]", key)))
    }
  })
}
