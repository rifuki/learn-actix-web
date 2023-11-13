/* * shared mutable state */
use actix_web::{HttpServer, App, web, Responder, HttpResponse};
use std::sync::Mutex;

struct AppStateWithCounter {
    counter: Mutex<i32>,
}

async fn index(data: web::Data<AppStateWithCounter>) -> impl Responder {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;

    HttpResponse::Ok()
        .append_header(("Content-Type", "application/json"))
        .body(format!(r#"{{"counter": {}}}"#, counter))
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0)
    });

    HttpServer::new(move || {

        App::new().
            app_data(counter.clone())
            .route("/counter", web::get().to(index))

    })
    .bind(("::", 80))?
    .run()
    .await
}
