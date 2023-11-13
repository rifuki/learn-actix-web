use actix_web::{
    HttpServer,
    App, 
    middleware::NormalizePath,
    web
};

use aw19_20_database::{
    items::scoped_items,
    establish_connection
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
        .expect("database URL must be set.");

    let pool = establish_connection(database_url);

    HttpServer::new(move || {
        App::new()
            .wrap(
                NormalizePath::trim()
            )
            .app_data(web::Data::new(pool.clone()))
            .configure(scoped_items)
    })
    .bind(("::", 80))?
    .run()
    .await
}
