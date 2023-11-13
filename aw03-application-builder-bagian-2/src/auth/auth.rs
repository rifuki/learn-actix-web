use actix_web::{web, HttpResponse};

pub fn scoped_auth(cfg: &mut web::ServiceConfig) -> () {
    cfg.service(
        web::resource("/auth")
            .route(web::get().to(|| async { HttpResponse::Ok().body("you're accessing /api/auth")}))
            .route(web::head().to(HttpResponse::MethodNotAllowed))
    );
}