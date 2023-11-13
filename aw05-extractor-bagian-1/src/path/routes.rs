use actix_web::web;

use crate::path::handler::{path1, path2, path3};

pub fn scoped_path(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/paths")
        .service(path1)
        .service(path2)
        .service(path3)
    );
}