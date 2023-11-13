use actix_web::{web, get, HttpResponse, http::header::ContentType};

#[get("/response")]
async fn index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .insert_header(("X-Hdr", "Sample"))
        .body("data")
}

pub fn scoped_response(cfg: &mut web::ServiceConfig) {
    cfg.service(index);
}
