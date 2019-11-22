use actix_session::Session;
use actix_web::{web, HttpRequest, HttpResponse};

use rustimate_service::AppConfig;

/// Available at `/testbed/{key}`
pub fn testbed_key(session: Session, cfg: web::Data<AppConfig>, key: web::Path<String>, req: HttpRequest) -> HttpResponse {
  crate::act(&session, &cfg, req, |ctx, router| {
    let k: &str = &key;
    match k {
      "dump" => rustimate_templates::testbed::dump(&ctx, router),
      "gallery" => rustimate_templates::testbed::gallery(&ctx, router),
      "prototype" => rustimate_templates::testbed::prototype(&ctx, router),
      "scroll" => rustimate_templates::testbed::scroll(&ctx, router),
      _ => Err(anyhow::anyhow!("Cannot find testbed matching [{}]", key))
    }
  })
}
