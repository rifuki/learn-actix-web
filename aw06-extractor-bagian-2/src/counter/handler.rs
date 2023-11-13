use actix_web::{web, get, HttpResponse, http::StatusCode};
use std::sync::{Arc, Mutex};

pub struct AppState {
    pub counter: Arc<Mutex<u32>>
}

#[get("/increment")]
pub async fn increment(app_state: web::Data<AppState>) -> HttpResponse {
    let mut counter = app_state.counter.lock().unwrap();
    *counter += 1;

    HttpResponse::build(StatusCode::OK)
        .insert_header(("Content-Type", "application/json"))
        .body(
            format!(
                r#"{{ "counter": {}}}"#, 
                *counter
            )
        )
}

#[get("/decrement")]
pub async fn decrement(app_state: web::Data<AppState>) -> HttpResponse {
    let mut counter = app_state.counter.lock().unwrap();

    /* * handle counter don't subtract below 0 */
    if *counter >= 1 {
        *counter -= 1;
    }

    HttpResponse::build(StatusCode::OK)
        .insert_header(("Content-Type", "application/json"))
        .body(
            format!(
                r#"{{ "counter": {}}}"#, 
                *counter
            )
        )
}
