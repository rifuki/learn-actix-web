use actix_web::{
    HttpServer, 
    App, 
    middleware, 
    get, 
    HttpResponse,
    dev::Service /* <- need this */ 
};
use futures_util::future::FutureExt; /* <- need this */

#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hello World")
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(
        env_logger::Env::new().default_filter_or("info")
    );
    HttpServer::new(|| {
        App::new()
            /* * middleware is executed from the bottom */
            /* * loggin */
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .wrap(middleware::NormalizePath::trim()) 
            /* *  end logging */
            /* * make simple middleware */
            .wrap_fn(|req, srv| { /* <- this middleware will be executed first */
                println!("Hi from start. You requested: {}", req.path());

                srv.call(req).map(| res | {
                    println!("Hi from response");
                    res
                })
            })
            /* * end make simple middleware */
            .service(index)
    })
    .bind(("::", 80))?
    .run()
    .await
}

