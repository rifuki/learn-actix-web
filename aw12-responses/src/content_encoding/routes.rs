use actix_web::web;

use crate::content_encoding::handler::index;

pub fn scoped_content_encoding(cfg: &mut web::ServiceConfig) {
    cfg.service(index);
}
