use actix_web::{HttpServer, App};

use aw09_url_dispatch_bagian_1::{
    resource_configuration::{
        scoped_resource_conf_prefix, 
        scoped_resource_conf_user, 
        scoped_resource_conf_path
    }, 
    scoping_routes::{
        scoped_scoping_routes, 
        scoped_scoping_routes_aw_additional
    }, 
    match_information::scoped_match_information
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            /* * resource configuration */
            .configure(scoped_resource_conf_prefix)
            .configure(scoped_resource_conf_user)
            .configure(scoped_resource_conf_path)
            /* * end resource configuration */
            /* * scoping configuration */
            .configure(scoped_scoping_routes)
            .configure(scoped_scoping_routes_aw_additional)
            /* * end scoping configuration */
            /* * matching information */
            .configure(scoped_match_information)
    })
    .bind(("::", 80))?
    .run()
    .await
}
