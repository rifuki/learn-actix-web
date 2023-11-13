use actix_web::web;

use crate::counter::handler::{increment, decrement};

pub fn scoped_counter(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/counter")
            .service(increment)
            .service(decrement)
    );
}