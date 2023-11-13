use actix_web::{web, guard, HttpResponse};

pub fn scoped_resource_conf_path(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/path")
            .route(
                web::route()
                    .guard(guard::Get())
                    .guard(guard::Header("content-type", "text/plain"))
                    .to(HttpResponse::Ok)
            )
    );
}