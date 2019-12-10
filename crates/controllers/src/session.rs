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
  crate::redir(&session, &cfg, req, |_ctx, router| {
    let es = EstimateSession::new(key.key.clone());
    cfg.session_svc().write_session(&es)?;
    router.route("session.join", &[&es.key()])
  })
}

/// Available at `/s/join`
pub(crate) fn join_link(session: Session, cfg: web::Data<AppConfig>, req: HttpRequest, key: web::Query<QueryStringKey>) -> HttpResponse {
  crate::redir(&session, &cfg, req, |_ctx, router| {
    let es = cfg.session_svc().read_session(&key.key)?;
    router.route("session.join", &[&es.key()])
  })
}

/// Available at `/s/{key}`
pub(crate) fn join(session: Session, cfg: web::Data<AppConfig>, req: HttpRequest, key: web::Path<String>) -> HttpResponse {
  crate::act(&session, &cfg, req, |ctx, router| {
    let es = cfg.session_svc().read_session(&key)?;
    rustimate_templates::session::detail(&ctx, router, &es)
  })
}
