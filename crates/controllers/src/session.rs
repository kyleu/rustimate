use actix_session::Session;
use actix_web::{web, HttpRequest, HttpResponse};

use rustimate_core::session::EstimateSession;
use rustimate_service::AppConfig;

#[derive(serde::Deserialize)]
pub(crate) struct QueryStringKey {
  key: String
}

/// Available at `/s/create`
pub(crate) fn create(session: Session, cfg: web::Data<AppConfig>, req: HttpRequest, key: web::Query<QueryStringKey>) -> HttpResponse {
  crate::redir(&session, &cfg, &req, |ctx| {
    let es = EstimateSession::new(key.key.clone());
    cfg.sessions().write().unwrap().write_session(&es)?;
    ctx.router().route("session.join", &[&es.key()])
  })
}

/// Available at `/s/join`
pub(crate) fn join_link(session: Session, cfg: web::Data<AppConfig>, req: HttpRequest, key: web::Query<QueryStringKey>) -> HttpResponse {
  crate::redir(&session, &cfg, &req, |ctx| {
    let es = {
      let svc = cfg.sessions().read().unwrap();
      svc.read_session(&key.key)?
    };
    ctx.router().route("session.join", &[&es.key()])
  })
}

/// Available at `/s/{key}`
pub(crate) fn join(session: Session, cfg: web::Data<AppConfig>, req: HttpRequest, key: web::Path<String>) -> HttpResponse {
  crate::act(&session, &cfg, &req, |ctx| {
    let es = {
      let svc = cfg.sessions().read().unwrap();
      svc.read_session(&key)?
    };
    rustimate_templates::session::detail(&ctx, &es)
  })
}
