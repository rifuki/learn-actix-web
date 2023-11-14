use actix_web::{
    error::ResponseError,
    http::StatusCode,
    HttpResponse
};

use sqlx::Error as PgError;
use serde_json::{
    Map as JsonMap,
    Value as JsonValue,
};
use validator::ValidationErrors;
use thiserror::Error as ThisError;

use std::convert::From;

#[derive(ThisError, Debug)]
pub enum Error {
    // 400
    #[error("BadRequest: {0}")]
    BadRequest(JsonValue),

    // 401
    #[error("Unauthorized: {0}")]
    Unauthorized(JsonValue),

    // 403
    #[error("Forbidden: {0}")]
    Forbidden(JsonValue),

    // 404
    #[error("Not Found: {0}")]
    NotFound(JsonValue),

    // 422
    #[error("Unprocessable: {0}")]
    UprocessableEntity(JsonValue),

    // 500
    #[error("Internal Server Error")]
    InternalServerError
}

impl ResponseError for Error {
   fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        match *self {
            Self::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
            Self::Unauthorized(ref message) => HttpResponse::Unauthorized().json(message),
            Self::Forbidden(ref message) => HttpResponse::Forbidden().json(message),
            Self::NotFound(ref message) => HttpResponse::NotFound().json(message),
            Self::UprocessableEntity(ref message) => {
                HttpResponse::build(StatusCode::UNPROCESSABLE_ENTITY)
                    .json(message)
            },
            Self::InternalServerError => HttpResponse::InternalServerError().json("internal server error")
        }
   } 
}

impl From<PgError> for Error {
    fn from(err: PgError) -> Self {
        match err {
            PgError::Database(db_err) => {
                eprintln!("Database Error: {:?}", db_err);
            
                Self::InternalServerError
            }
            _ => Self::InternalServerError
        }
    }
}

impl From<ValidationErrors> for Error {
    fn from(errors: ValidationErrors) -> Self {
        let mut err_map = JsonMap::new();

        for (field, errors) in errors.field_errors().iter() {
            let errors: Vec<JsonValue> = errors
                .iter()
                .map(| error | {
                    serde_json::json!(error.message)
                })
                .collect();

            err_map.insert(field.to_string(), serde_json::json!(errors));
        }

        Self::UprocessableEntity(serde_json::json!({
            "errors": err_map
        }))
    }
}