use actix_web::{web::JsonConfig, error, HttpResponse};

pub fn json_config() -> JsonConfig {
    JsonConfig::default()
        .limit(100) /* <- using bit units */
        .error_handler(|err, _req| {
            error::InternalError::from_response(
                err,
                HttpResponse::Conflict().body("limit reached")
            ).into()
        })
}