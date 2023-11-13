use actix_web::web;

use crate::json::{
    config::json_config,
    handler::{
        json1,
        json2
    }
};

pub fn scoped_json(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/json")
            .service(json1)
            .service(
                web::resource("/v2")
                .app_data(json_config())
                .route(web::post().to(json2))
            )
    );
}