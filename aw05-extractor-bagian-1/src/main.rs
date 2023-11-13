use actix_web::{HttpServer, App, middleware::NormalizePath, web};

use aw05_extractor_bagian_1::{
    path::scoped_path,
    query::scoped_query,
    json::scoped_json, 
    urlencoded_form::scoped_urlencoded_form
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .wrap(
            NormalizePath::trim()
        )
        /* * Path Extractor */
        .configure(scoped_path)
        /* * Query Extractor */
        .configure(scoped_query)
        /* * JSON Extractor */
        .configure(scoped_json)
        /* * URL-Encoded Forms */
        .configure(scoped_urlencoded_form)
        /* * handle unregistered endpoint */
        .default_service(
            web::route()
                .to(|| async {
                    "Page Not Found"
                }
            )
        )
    })
    .workers(1)
    .bind(("::", 80))?
    .run()
    .await
}
