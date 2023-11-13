use actix_web::{web::FormConfig, error, HttpResponse};

pub fn urlencoded_form_config() -> FormConfig {
    FormConfig::default()
        .limit(1000)
        .error_handler(|err, _req| {
            error::InternalError::from_response(
                err,
                HttpResponse::Conflict().body("limit reached")
            ).into()
        })
}   