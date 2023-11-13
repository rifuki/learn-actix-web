use actix_web::{HttpResponse, web};

async fn prefix() -> HttpResponse {
    HttpResponse::Ok().body("Prefix")
}

pub fn scoped_resource_conf_prefix(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/prefix")
        .to(prefix)   
    );
}