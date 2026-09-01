//! Application factory for the `example-testing` app.
//!
//! Having a function that configures our app makes it easy to build it from
//! tests without having to create an HTTP server.

use actix_web::{error, web, HttpRequest, HttpResponse, Responder};

/// Registers every route of the application on the given service config.
///
/// Both the binary and the integration tests build their `App` from this, so
/// the routing table under test is exactly the one that is served.
pub fn configure(cfg: &mut web::ServiceConfig) {
    // Routes are registered as resources so that a request whose path matches
    // but whose method does not is answered with `405 Method Not Allowed`
    // rather than `404 Not Found`.
    cfg.service(web::resource("/").route(web::get().to(hello_world)))
        .service(
            web::resource("/json")
                .app_data(json_config())
                .route(web::post().to(json)),
        )
        .service(
            web::resource("/requires-connect-info")
                .route(web::get().to(requires_connect_info)),
        );
}

/// Maps JSON extraction failures onto the same statuses the original
/// implementation returned: `415` when the `Content-Type` is missing or wrong,
/// `400` when the body itself cannot be deserialized.
fn json_config() -> web::JsonConfig {
    web::JsonConfig::default().error_handler(|err, _req| match &err {
        error::JsonPayloadError::ContentType => {
            error::InternalError::from_response(err, HttpResponse::UnsupportedMediaType().finish())
                .into()
        }
        _ => error::InternalError::from_response(err, HttpResponse::BadRequest().finish()).into(),
    })
}

async fn hello_world() -> impl Responder {
    "Hello, World!"
}

async fn json(payload: web::Json<serde_json::Value>) -> impl Responder {
    web::Json(serde_json::json!({ "data": payload.into_inner() }))
}

async fn requires_connect_info(req: HttpRequest) -> HttpResponse {
    // Mirrors axum's `ConnectInfo` extractor: when the peer address is not
    // available the request is rejected instead of being served.
    match req.peer_addr() {
        Some(addr) => HttpResponse::Ok().body(format!("Hi {addr}")),
        None => HttpResponse::InternalServerError().finish(),
    }
}
