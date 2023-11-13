use actix_web::web;

use crate::streaming_request::handler::index;

pub fn scoped_streaming_request(cfg: &mut web::ServiceConfig) {
    cfg.service(index);   
}