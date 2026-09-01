//! Run with
//!
//! ```not_rust
//! cargo test -p example-testing
//! ```

use actix_web::{middleware::Logger, App, HttpServer};
use example_testing::configure;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(
        env_logger::Env::new().default_filter_or(concat!(env!("CARGO_CRATE_NAME"), "=debug,info")),
    );

    let server = HttpServer::new(|| {
        // We can still add middleware
        App::new().wrap(Logger::default()).configure(configure)
    })
    .bind(("127.0.0.1", 3000))?;

    log::debug!("listening on {}", server.addrs()[0]);

    server.run().await
}
