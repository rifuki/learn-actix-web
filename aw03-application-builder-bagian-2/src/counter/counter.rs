use actix_web::{web, Responder, HttpResponse};
use std::cell::Cell;

struct MyData {
    counter: Cell<usize>
}

async fn handler(data: web::Data<MyData>) -> impl Responder {
    data.counter.set(data.counter.get() + 1);
    format!("{}", data.counter.get())
}

pub fn scoped_counter(cfg: &mut web::ServiceConfig) -> () {
    cfg.service(
        web::resource("/counter")
            .app_data(web::Data::new(MyData {counter: Default::default()}))
            .route(web::get().to(handler))
            .route(web::head().to(HttpResponse::MethodNotAllowed))
    );
}