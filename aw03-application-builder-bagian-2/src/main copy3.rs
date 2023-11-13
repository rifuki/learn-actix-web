/* * application guard */
use actix_web::{HttpServer, App, web, Responder, HttpResponse, guard};

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .service(
                web::scope("/guard")
                    .guard(guard::Get())
                    .route("/hey", web::to(manual_hello))
            )
    })
    .bind(("::", 80))?
    .run()
    .await
}
