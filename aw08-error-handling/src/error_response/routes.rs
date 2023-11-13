use actix_web::web;

use crate::error_response::handler::error_response;

pub fn scoped_error_response(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/error_response")
            .route(web::get().to(error_response))
    );
}