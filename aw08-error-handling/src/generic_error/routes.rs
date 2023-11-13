use actix_web::web;

use crate::generic_error::handler::generic_error;

pub fn scoped_generic_error(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/generic_error")
            .route(web::post().to(generic_error))   
    );
} 