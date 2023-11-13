/* * configure */
use actix_web::{HttpServer, App, web, HttpResponse, middleware::NormalizePath};

use aw03_application_builder_bagian_2::{
    auth::scoped_auth, 
    counter::scoped_counter, 
    security::scoped_security
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .wrap(
                NormalizePath::trim()
            )
            .configure(scoped_security)
            .service(
                web::scope("/api")
                .configure(scoped_auth)
                .configure(scoped_counter)
            )
            .route("/", web::get().to(|| async { HttpResponse::Ok().body("you're accessing /")}))
    })
    .workers(1)
    .bind(":::80")?
    // .bind(("::", 80))?
    .run()
    .await
}
