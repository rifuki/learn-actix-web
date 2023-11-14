use actix_web::HttpResponse;

/// Get ping API response
/// 
/// Healthy API should always response OK
#[utoipa::path(
    get,
    path = "/ping",
    tag = "ping",
    responses(
        (status = 200, description = "Success"),
        (status = 404, description = "Not Found")
    )
)]

pub async fn ping() -> HttpResponse {
    HttpResponse::Ok().finish()
}