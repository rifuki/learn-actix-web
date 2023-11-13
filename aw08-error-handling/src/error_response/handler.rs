use actix_web::{Result, error};
use derive_more::{Display, Error};

#[derive(Display, Error, Debug)]
#[display(fmt = "CUSTOM ERROR: {}", msg)]
pub struct MyCustomError {
    msg: &'static str
}

impl error::ResponseError for MyCustomError {}

pub async fn error_response() -> Result<&'static str, MyCustomError> {
    Err(
        MyCustomError { msg: "there is custom error message" }
    )
}