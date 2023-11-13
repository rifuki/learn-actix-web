use actix_web::{HttpServer, App, middleware};
use aw16_static_files::individual_file::scoped_individual_file;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            /* * individual file */
            .configure(scoped_individual_file)
            /* * end individual file */
            .wrap(
                middleware::DefaultHeaders::new()
                    .add(
                        ("X-Content-Type-Options", "nosniff")
                    )
            )
            /* * directory */
            .service(
                actix_files::Files::new(
                    "/static", "."
                ).show_files_listing()
            )
            .default_service(
                actix_files::Files::new(
                    "/", "."
                )
                .index_file("index.html")
                .show_files_listing()
            )
            /* * end end directory */
    })
    .bind(("::", 80))?
    .run()
    .await
}
