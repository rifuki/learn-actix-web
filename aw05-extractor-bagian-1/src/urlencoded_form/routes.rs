use actix_web::web;

use crate::urlencoded_form::{
    config::urlencoded_form_config,
    handler::{
        form1,
        form2
    }
};

pub fn scoped_urlencoded_form(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/forms")
            .service(form1)
            .service(
                web::resource("/v2")
                .app_data(urlencoded_form_config())
                .route(web::post().to(form2))
            )
    );   
}