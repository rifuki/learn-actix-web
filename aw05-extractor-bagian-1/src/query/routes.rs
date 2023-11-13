use actix_web::web;

use crate::query::handler::query;

pub fn scoped_query(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/queries")
            .service(query)
    );
}