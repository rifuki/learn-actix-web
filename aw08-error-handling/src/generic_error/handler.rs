use actix_web::{
    error, 
    HttpResponse, 
    body::BoxBody, 
    http::{
        StatusCode, 
        header::ContentType
    }, 
    Result, web
};
use derive_more::Display;

#[derive(Debug, Display)]
pub enum MyCustomError {
    #[display(fmt = "invalid input provided.")]
    InvalidInput,

    #[display(fmt = "an internal error occured.")]
    InternalError
}

impl error::ResponseError for MyCustomError {
    fn error_response(&self) -> HttpResponse<BoxBody> {
        let status_code = self.status_code();
        let error_message = self.to_string();

        /* * error logging */
        use log::error;
        error!("{} - Status code: {}", error_message, status_code);
        /* * end error logging */

        HttpResponse::build(status_code)
            .content_type(ContentType::html())
            .body(error_message)
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            Self::InvalidInput => StatusCode::BAD_REQUEST,
            Self::InternalError => StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

fn do_something_fails() -> Result<&'static str, MyCustomError> {
    Err(
        MyCustomError::InternalError
    )
}


#[derive(serde::Deserialize)]
pub struct MyCustomJsonPayload {
    value: String
}

pub async fn generic_error(
    payload: web::Json<MyCustomJsonPayload>
) -> Result<&'static str, MyCustomError> {
    if payload.value.is_empty() {
        return Err(MyCustomError::InvalidInput)
    } 

    do_something_fails()?;
    Ok("success")
}