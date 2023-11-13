use actix_web::{error, 
    HttpResponse, 
    body::BoxBody, 
    http::{
        header::ContentType,
        StatusCode
    },
    get
};

use derive_more::Display;

#[derive(Display, Debug)]
pub enum MyCustomError {
    #[display(fmt = "internal error")]
    InternalError,

    #[display(fmt = "bad request")]
    BadClientData,

    #[display(fmt = "timeout")]
    Timeout,
}

impl error::ResponseError for MyCustomError {
    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            Self::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            Self::BadClientData => StatusCode::BAD_REQUEST,
            Self::Timeout => StatusCode::GATEWAY_TIMEOUT
        }
    }
}

#[get("/internal_error")]
pub async fn internal_error() -> Result<&'static str, MyCustomError> {
    Err(MyCustomError::InternalError)
}

#[get("/bad_client_data")]
pub async fn bad_client_data() -> Result<String, MyCustomError> {
    Err(MyCustomError::BadClientData)
}

#[get("/timeout")]
pub async fn timeout() -> Result<HttpResponse, MyCustomError> {
    Err(MyCustomError::Timeout)
}
