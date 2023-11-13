use actix_web::web;

use crate::streaming_res_body::handler::stream;

pub fn scoped_streaming_res_body(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/streaming")
            .service(stream)
    );
}