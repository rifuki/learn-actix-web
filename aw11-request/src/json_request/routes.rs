use actix_web::{web, guard::{self, Header}};

use crate::json_request::handler::index;

pub fn scoped_json_request(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/json_request")
            .guard(guard::Post())
            .guard(Header("Content-Type", "application/json"))
            .to(index)
    );
}