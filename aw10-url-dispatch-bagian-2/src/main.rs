use actix_web::{HttpServer, App, middleware::NormalizePath, web, HttpResponse, guard};

use aw10_url_dispatch_bagian_2::{
    generating_resource_urls::generating_resource_urls, 
    external_resources::scoped_external_resource,
    da_alternate_resource_name::scoped_alternate_resource_name, 
    path_normalization::scoped_path_normalization,
    custom_guards::scoped_custom_guards
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(
                NormalizePath::trim() /* <-0 put there */
            )
            /* * generating resource urls */
            .configure(generating_resource_urls)
            /* * end generating resource urls */
            /* * external resources */
            .configure(scoped_external_resource)
            /* * end external resources */
            /* * DA - alternate resource name */
            .configure(scoped_alternate_resource_name)
            /* * end DA - alternate resource name */
            /* * path normalization */
            .configure(scoped_path_normalization)
            /* * end path normalization */
            /* * custom guards */
            .configure(scoped_custom_guards)
            /* * end custom guards */
            /* * modifiying guard values (notallowed) */
            .route(
                "/notallowed",
                web::route()
                    .guard(guard::Not(guard::Get()))
                    .to(|| async { "exclude GET method is allowed" })
             )
            /* * end modifiying guard values (notallowed)*/
            /* * changing the default Not Found response */
            .default_service(
                web::route()
                    .to(|| async {
                        HttpResponse::NotFound()
                            .body("URL not found")
                    })
            )
            /* * end changing the default Not Found response */
    })
    .bind(("::", 80))?
    .run()
    .await
}
