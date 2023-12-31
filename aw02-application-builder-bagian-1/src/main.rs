/* * state */
use actix_web::{HttpServer, App, web, Responder, get};

struct AppState {
    app_name: String
}

// #[get("/")] /* <- comment this if use alternative */
async fn index(data: web::Data<AppState>) -> impl Responder {
    let app_name = &data.app_name;
    format!("Hello, {}!", app_name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState {
                app_name: String::from("Rust Programming Language")
            }))
            // .service(index)
            .route("/hey", web::get().to(index)) /* <- alternative */
    })
    .bind(("::", 80))?
    .run()
    .await
}
