use aw21_28_restful_api::{
    settings::{
        get_app_mode,
        DatabaseSettings
    },
    server::{
        Application, 
        get_connection_pool
    }, 
};
use sqlx::{PgPool, PgConnection, Connection, Executor};
use chrono::Local;
use uuid::Uuid;
use wiremock::MockServer;

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
    pub port: u16,
    pub test_server: MockServer
}

pub async fn start_test_server() -> TestApp {
    let test_server = MockServer::start().await;
    
    let current_time = Local::now();
    let time_prefix = current_time.format("%Y%m%d%H%M%S").to_string();

    let configuration = {
        let mut cfg = get_app_mode().expect("Failed to read configuration");

        cfg.database.database_name = format!(
            "{}_{}_{}", 
            "aw_api_test".to_string(), 
            time_prefix, 
            Uuid::new_v4().to_string()
        );
        cfg.application.port = 0;
        cfg.test_client.base_url = test_server.uri();

        cfg
    };

    /* * create and migrate the database */
    configure_database(&configuration.database).await;

    let app = Application::build_app(configuration.clone())
        .await
        .expect("failed to build application");

    let application_port = app.port();
    let address = format!("http://127.0.0.1:{}", app.port());

    let _ = actix_web::rt::spawn(app.run_app());

    TestApp { 
        address, 
        db_pool: get_connection_pool(&configuration.database),
        port: application_port,
        test_server
    }
}

async fn configure_database(config: &DatabaseSettings) -> PgPool {
    println!("config: {:?}", config);
    // let mut connection = PgConnection::connect_with(&config.parse_connection_string())
    let solve_postgresql_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        config.username,
        config.password,
        config.host,
        config.port,
        "postgres"
    );

    let mut connection = PgConnection::connect(&solve_postgresql_url)
        .await
        .expect("failed to connect Postgres DB");

    connection.execute(
        format!(
            r#"CREATE DATABASE "{}";"#,
            config.database_name
        ).as_str()
    )
    .await
    .expect("failed to create database");

    let connection_pool = PgPool::connect_with(config.database_connection_string())
        .await
        .expect("failed to connect to Postgres DB");

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("failed to migrate database");

    return connection_pool;
}