use actix_web::{web, guard, HttpResponse};

pub fn scoped_resource_conf_user(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/user/{name}")
            .name("user_detail")
            .guard(guard::Header("content-type", "application/json"))
            .route(web::get().to(HttpResponse::Ok))
            .route(web::put().to(HttpResponse::Ok))
    );
}