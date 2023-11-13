use actix_web::{web, guard, HttpResponse};

async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hello World")
}

pub fn scoped_scoping_routes_aw_additional(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(
                web::resource("/hello")
                    .route(
                        web::route()
                            .guard(guard::Get())
                            .to(index)
                    )
            )
    );
}