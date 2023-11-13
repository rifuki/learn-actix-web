use actix_web::{guard::{Guard, GuardContext}, http, web, HttpResponse};

struct ContentTypeHeader;
impl Guard for ContentTypeHeader {
    fn check(&self, ctx: &GuardContext<'_>) -> bool {
        ctx.head()
            .headers()
            .contains_key(http::header::CONTENT_TYPE)
    }
}

async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Custom Guards")
}

pub fn scoped_custom_guards(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/custom_guards")
            .guard(ContentTypeHeader)
            .to(index)
    );
}