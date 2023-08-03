//! src/lib.rs

use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use actix_web::dev::Server;
use std::net::TcpListener;


async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

pub fn run(address: TcpListener) -> Result<Server, std::io::Error> {

    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
    })
    .listen(address)?
    .run();

    Ok(server)
}