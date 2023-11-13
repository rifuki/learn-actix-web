/* * writing an application */
use actix_web::{HttpServer, App, web, Responder};

async fn index() -> impl Responder {
    "Hello index.html"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                 web::scope("/app")
                    .route("/index.html", web::get().to(index)) /* <- /app/index.html */
            )
    })
    .bind(("::", 80))?
    .run()
    .await
}
