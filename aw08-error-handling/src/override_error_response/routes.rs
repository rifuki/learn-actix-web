use actix_web::web;

use crate::override_error_response::handler::{internal_error, bad_client_data, timeout};

pub fn scoped_override_error_response(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/override_error_response")
            .service(internal_error)
            .service(bad_client_data)
            .service(timeout)
    );
}