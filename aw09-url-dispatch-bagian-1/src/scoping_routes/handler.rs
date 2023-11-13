use actix_web::{HttpResponse, get, web};

#[get("/show")]
pub async fn show_users() -> HttpResponse {
    HttpResponse::Ok().body("Show users")
}

#[get("/show/{id}")]
pub async fn user_detail(path: web::Path<(u32,)>) -> HttpResponse {
    HttpResponse::Ok().body(
        format!(
            "User detail: {}",
            path.into_inner().0
        )
    )
}