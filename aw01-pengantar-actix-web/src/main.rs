/* * getting started */
use actix_web::{HttpServer, App, post, web, get, Responder, HttpResponse};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World")
}

#[get("/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello, {}!", &name.into_inner())
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(greet)
            .route("/hey", web::post().to(manual_hello))
            .default_service(web::route().to(|| async { HttpResponse::NotFound().finish() } ))
    })
    .workers(1)
    .bind(("::", 80))?
    .run()
    .await
}
