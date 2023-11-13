use actix_web::web;

use crate::error_helpers::handler::bad_gateway;

pub fn scoped_error_helpers(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/error_helpers")
            .route("/badgateway", web::get().to(bad_gateway))
    );
}