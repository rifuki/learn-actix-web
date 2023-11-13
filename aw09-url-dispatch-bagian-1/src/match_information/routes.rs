use actix_web::web;

use crate::match_information::handler::version;

pub fn scoped_match_information(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/match_information")
            .service(version)
    );
}