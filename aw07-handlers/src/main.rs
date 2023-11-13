use actix_web::{HttpServer, App};

use aw07_handlers::{
    custom_response::scoped_custom_response,
    streaming_res_body::scoped_streaming_res_body,
    either::scoped_either
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        /* * custom response  */
        .configure(scoped_custom_response)
        /* * transimit data respons into chunks continously */
        .configure(scoped_streaming_res_body)
        /* * different return type (either) */
        .configure(scoped_either)
    })
    .bind(("::", 80))?
    .run()
    .await
}
