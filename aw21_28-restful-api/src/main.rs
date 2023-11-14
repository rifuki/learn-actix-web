use aw21_28_restful_api::{
    settings::get_app_mode,
    server::Application
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let app_setting = get_app_mode().expect("failed to read configuration file");

    let app = Application::build_app(app_setting).await?;

    app.run_app().await?;

    Ok(())
}