use actix_web::{dev::Server, HttpServer, App, web};
use std::net::TcpListener;

use crate::routes::index as api;

pub fn start(listener: TcpListener) -> std::io::Result<Server> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(api))
            .route("/", web::post().to(api))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
