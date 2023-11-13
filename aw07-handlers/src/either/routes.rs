use actix_web::web;

use crate::either::handler::either;

pub fn scoped_either(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/either")
            .route("", web::get().to(either))
    );
}