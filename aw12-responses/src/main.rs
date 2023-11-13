use actix_web::{HttpServer, App, middleware};

use aw12_responses::{
    response::scoped_response,
    json_response::scoped_json_response,
    content_encoding::scoped_content_encoding,
    already_compressed_body::scoped_already_compressed_body
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(
                middleware::Compress::default() /* default content encoding compression */
            )
            /* * response */
            .configure(scoped_response)
            /* * json response */
            .configure(scoped_json_response)
            /* * content encoding */
            .configure(scoped_content_encoding)
            /* * already compressed body */
            .configure(scoped_already_compressed_body)
    })
    .bind(("::", 80))?
    .run()
    .await
}
