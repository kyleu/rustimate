use actix_web::web::{get, post, resource};

/// Creates reverse routing table for [actix-web](actix-web)
pub fn add_routes(s: &mut actix_web::web::ServiceConfig) {
  let _ = s
    // Home
    .service(resource("/").name("home").route(get().to(crate::home::index)))
    .service(resource("/health").name("health").route(get().to(crate::home::health)))
    .service(resource("/profile").name("profile").route(get().to(crate::home::profile)).route(post().to(crate::home::profile_post)))
    .service(resource("/settings").name("settings").route(get().to(crate::home::settings)).route(post().to(crate::home::settings_post)))

    // Testbed
    .service(resource("/testbed/{key}").name("testbed.detail").route(get().to(crate::testbed::testbed_key)))

    // Rustimate Sessions
    .service(resource("/s/create").name("session.create").route(get().to(crate::session::create)))
    .service(resource("/s/join").name("session.join_link").route(get().to(crate::session::join_link)))
    .service(resource("/s/{key}").name("session.join").route(get().to(crate::session::join)))
    .service(resource("/s/{key}/connect").name("connect").route(get().to(crate::websocket::connect)))

    // Static
    .service(resource("/favicon.ico").name("favicon").route(get().to(crate::static_file::favicon)))
    .service(resource("/static/{filename:.*}").name("static").route(get().to(crate::static_file::static_file)))

    // Catch-all
    .service(resource("/{path:.*}").name("missing").to(crate::not_found));
}
