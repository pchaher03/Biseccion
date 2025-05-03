use actix_web::{App, HttpServer};
mod controllers;
mod models;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(controllers::solve_polynomial)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
