use actix_web::{Either, HttpResponse, Error};

fn is_bad() -> bool {
    true
}

type RegisterResult = Either<HttpResponse, Result<&'static str, Error>>;
pub async fn either() -> RegisterResult {
    if is_bad() {
        Either::Left(HttpResponse::BadRequest().body("bad data"))
    } else {
        Either::Right(
            Ok("success")
        )
    }
}