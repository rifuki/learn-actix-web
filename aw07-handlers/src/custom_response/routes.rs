use actix_web::web;

use crate::custom_response::handler::custom_res_json;

pub fn scoped_custom_response(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/custom")
            .service(custom_res_json)
    );
}