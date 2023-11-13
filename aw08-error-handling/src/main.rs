use actix_web::{
    HttpServer, 
    App, 
    middleware::Logger
};

use aw08_error_handling::{
    error_response::scoped_error_response,
    override_error_response::scoped_override_error_response,
    error_helpers::scoped_error_helpers, 
    generic_error::scoped_generic_error
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info"); /* <- logging from rust */
    std::env::set_var("RUST_BACKTRACE", "1"); /* <- logging from rust */
    env_logger::init(); /* <- !FOR ACTIVATED LOGGING! (must be set) */

    HttpServer::new(|| {
        App::new()
            .wrap(
                Logger::default() /* <- for more log from actix (routes, statuscode) need rust_log activated */
            ) 
            /* * error response */
            .configure(scoped_error_response)
            /* * override error response */
            .configure(scoped_override_error_response)
            /* * error helpers */
            .configure(scoped_error_helpers)
            /* * generic error */
            .configure(scoped_generic_error)
    })
    .bind(("::", 80))?
    .run()
    .await
}
