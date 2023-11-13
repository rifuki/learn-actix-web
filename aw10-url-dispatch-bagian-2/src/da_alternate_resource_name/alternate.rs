use actix_web::{web, Responder, get, HttpResponse, HttpRequest, http::header};

#[get("/show/{usename}", name = "show_users")]
async fn show_users(req: HttpRequest) -> impl Responder {
    let username = req.match_info().load::<String>().unwrap();

    HttpResponse::Ok()
        .body(
            format!("showing user with username: {}", username)
        )
}

#[get("/generate")]
pub async fn generate(req: HttpRequest) -> HttpResponse {
    let url = req.url_for("show_users", ["rifuki"]);

    match url {
        Ok(url) => HttpResponse::SeeOther()
            .insert_header((header::LOCATION, url.as_str()))
            .finish(),
        Err(_) => HttpResponse::InternalServerError().body("Failed to generate URL")
    }
} 

pub fn scoped_alternate_resource_name(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/alternate")
        .service(show_users)
        .service(generate)
    );
}