use actix_web::{error, Result};

struct MyCustomError {
    msg: &'static str
}

pub async fn bad_gateway() -> Result<&'static str> {
    let result: Result<&'static str, MyCustomError> = Err(
        MyCustomError {
            msg: "will bad gateway"
        }
    );

    Ok(result.map_err(|e| error::ErrorBadGateway(e.msg))?)
} 