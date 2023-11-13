use actix_web::{get, HttpResponse, http::header::ContentEncoding};

#[get("/content_encoding")]
pub async fn index() -> HttpResponse {
    HttpResponse::Ok()
        .insert_header(ContentEncoding::Identity) /* <- for disable default compression response */
        .body("data")
}
