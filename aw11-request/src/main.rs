use actix_web::{HttpServer, App};

use aw11_request::{
    json_request::scoped_json_request, 
    manually_load_payload::scoped_manually_load_payload,
    content_encoding::scoped_content_encoding, 
    streaming_request::scoped_streaming_request
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            /* * scoped json request */
            .configure(scoped_json_request)
            /* * manual load payload into memory then desearialize */
            .configure(scoped_manually_load_payload)
            /* * content encoding */
            .configure(scoped_content_encoding)
            /* * streaming request */
            .configure(scoped_streaming_request)
    })
    .bind(("::", 80))?
    .run()
    .await
}
