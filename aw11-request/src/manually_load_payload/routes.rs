use actix_web::web;

use crate::manually_load_payload::handler::manual_index;

pub fn scoped_manually_load_payload(cfg: &mut web::ServiceConfig) {
    cfg.service(
       manual_index 
    );
}