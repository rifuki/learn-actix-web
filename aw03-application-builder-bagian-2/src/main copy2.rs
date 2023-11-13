/* * scoped to compose application */
use actix_web::{HttpServer, App, web, Responder, HttpResponse, get, post};

#[post("/login")]
async fn login() -> impl Responder {
    HttpResponse::Ok().body("Login")
}

#[get("/logout")]
async fn logout() -> impl Responder {
    HttpResponse::Ok().body("Logout")
}

#[get("")]
async fn get_all_buyers() -> impl Responder {
    HttpResponse::Ok().body("All Buyers")
}

#[get("/{id}")]
async fn get_buyer_by_id(id: web::Path<i32>) -> impl Responder {
    HttpResponse::Ok()
        .body(format!("Get buyer with id: {}", id.into_inner()))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        
        let scoped_auth= web::scope("/auth")
            .service(login)
            .service(logout);
        let scoped_buyers = web::scope("/buyers")
            .service(get_all_buyers)
            .service(get_buyer_by_id);

        App::new().
            app_data(counter.clone())
            .service(scoped_auth)
            .service(scoped_buyers)

    })
    .bind(("::", 80))?
    .run()
    .await
}
