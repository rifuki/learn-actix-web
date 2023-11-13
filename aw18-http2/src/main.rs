use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use actix_web::{HttpServer, App, Responder, HttpRequest, web};

async fn index(req: HttpRequest) -> impl Responder {
    println!("{:#?}", req.head());

    "Hello World"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut builder = SslAcceptor::mozilla_intermediate(
        SslMethod::tls()
    ).unwrap();

    builder
        .set_private_key_file("key.pem", SslFiletype::PEM).unwrap();

    builder
        .set_certificate_chain_file("cert.pem").unwrap();
    
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
    })
    .bind_openssl(("::", 443), builder)?
    .run()
    .await
}
