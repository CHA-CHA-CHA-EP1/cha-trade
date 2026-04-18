use actix_web::{App, HttpServer, web};

use account::handler;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("server is listing on http://0.0.0.0:8080");
    let server = HttpServer::new(move || {
        App::new().route(
            "/health-check",
            web::get().to(handler::health_check_handler::health_check),
        )
    })
    .bind(("0.0.0.0", 8080))?
    .run();

    server.await
}
