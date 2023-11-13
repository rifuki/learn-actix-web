use actix_web::{HttpServer, App, web};
use std::sync::{Arc, Mutex};

use aw06_extractor_bagian_2::counter::{AppState, scoped_counter};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        counter: Arc::new(Mutex::new(0))
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            /* * application state extractor */
            .configure(scoped_counter)
    })
    .bind(("::", 80))?
    .run()
    .await
}
