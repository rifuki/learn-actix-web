use actix_web::{
    web, 
    HttpResponse, 
    middleware::NormalizePath /* <-0 NormalizePath must be put in app layer \ not in service layer */
};

async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Path normalization")
}

pub fn scoped_path_normalization(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/path_normalization")
            .to(index)
    );
}