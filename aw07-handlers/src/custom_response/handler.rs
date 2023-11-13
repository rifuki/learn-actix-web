use actix_web::{get, Responder, body::BoxBody, HttpRequest, HttpResponse, http::header::ContentType};
use serde::Serialize;

#[derive(Serialize)]
struct MyObj {
    name: &'static str
}

impl Responder for MyObj {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap(); /* <- comment if use .json method */

        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body) /* <- comment if use .json method */
            // .json(self) /* <- .json method instant way for make res json */
    }
}

#[get("/json")]
pub async fn custom_res_json() -> impl Responder {
    MyObj { name: "rifuki" }
}