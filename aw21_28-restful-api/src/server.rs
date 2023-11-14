// #![allow(unused)]

use crate::{
        schemas::{
            UserRegister,
            UserLogin,
            UserUpdate,
            UserDelete,
        },
        routes::{
            __path_ping,
            __path_register,
            __path_login,
            __path_update,
            __path_delete,
            ping,
            register,
            login,
            update,
            delete
        }
};

use actix_web::{
    HttpServer,
    dev::Server,
    web,
    App, 
};

use std::net::TcpListener;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use sqlx::{
    PgPool,
    postgres::PgPoolOptions
};

use crate::settings::{
    Settings, 
    DatabaseSettings
};

pub fn get_connection_pool(
    configuration: &DatabaseSettings
) -> PgPool {
    PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(3))
        .connect_lazy_with(configuration.database_connection_string())
}

pub struct Application {
    port: u16,
    server: Server
}

impl Application {
    pub async fn build_app(configuration: Settings) -> Result<Self, std::io::Error> {
        let connection_pool = get_connection_pool(&configuration.database);

        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );

        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr().unwrap().port();
        let server = start(listener, connection_pool)?;

        Ok(
            Self {
                port,
                server
            }
        )
    }   

    pub async fn run_app(self) -> Result<(), std::io::Error> {
        self.server.await
    } 

    pub fn port(&self) -> u16 {
        self.port
    }
}

pub fn start(
    listener: TcpListener, 
    db_pool: PgPool
) -> Result<Server, std::io::Error> {
    #[derive(OpenApi)]
    #[openapi(
        paths(ping, register, login, update, delete),
        info(
            title = "Actix-web RESTful",
            version = "0.1.0",
            description = "My fifth Rust Backend with _`Actix-web`_",
            license(name = "MIT", url = "github.com"),
            contact(name = "rifuki", url = "github.com/rifuki", email = "mahomarifuki@gmail.com")
            
        ),
        components(
            schemas(
                UserRegister, UserLogin, UserUpdate, UserDelete
            )
        )
    )]
    struct ApiDoc;

    let db_pool_data = web::Data::new(db_pool);

    let server = HttpServer::new(move || {
        App::new()
            .app_data(db_pool_data.clone())
            .service(
                SwaggerUi::new(
                    "/apidoc/{_:.*}"
                )
                    .url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
            .route("/ping", web::get().to(ping))
            .service(
                web::scope("/api/v1")
                    .service(
                        web::resource("users/register")
                            .route(web::post().to(register))  
                    )
                    .service(
                        web::resource("users/login")
                            .route(web::post().to(login))  
                    )
                    .service(
                        web::resource("users/update")
                            .route(web::put().to(update))  
                    )
                    .service(
                        web::resource("users/delete")
                            .route(web::delete().to(delete))  
                    )
            )
    })
    .listen(listener)?
    .run();

    Ok(server)

}