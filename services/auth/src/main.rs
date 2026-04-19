use actix_web::{
    App, HttpServer,
    web::{self, route},
};

use auth::handler;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(move || {
        App::new().route(
            "/health-check",
            web::get().to(handler::health_check_handler::health_check),
        )
    })
    .bind(("0.0.0.0", 8081))?
    .run();

    server.await
}
