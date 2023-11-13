/* * http server tls */
#![allow(unused_doc_comments)]
use actix_web::{HttpServer, App, get, Responder, HttpResponse};
use openssl::ssl::{SslAcceptor, SslMethod, SslFiletype};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("welcome https")
}
/** rustls and openssl */
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut builder = SslAcceptor::mozilla_intermediate(
        SslMethod::tls()
    )
        .unwrap();

    /** set private key file */
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();

    /** set certificate file */
    builder
        .set_certificate_chain_file("cert.pem")
        .unwrap();

    HttpServer::new(|| {
        App::new()
            .service(index)
    })
    .bind_openssl(("::", 443), builder)?
    .run()
    .await
}