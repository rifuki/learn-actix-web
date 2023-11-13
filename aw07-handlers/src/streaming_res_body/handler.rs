use actix_web::{HttpResponse, web, Error, get};
use futures::{future::ok, stream::once};

#[get("")]
pub async fn stream() -> HttpResponse {
    let body = once(
        ok::<_, Error>(web::Bytes::from_static(b"Hello World"))
    );

    HttpResponse::Ok()
        .content_type("application/json")
        .streaming(body)
}