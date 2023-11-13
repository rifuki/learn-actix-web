use actix_web::{web, HttpResponse};

pub fn scoped_security(cfg: &mut web::ServiceConfig) -> () {
    cfg.service(
        web::resource("/security")
            .route(web::get().to(|| async { HttpResponse::Ok().body("you're accessing security")}))
            .route(web::head().to(HttpResponse::MethodNotAllowed))
    );
}