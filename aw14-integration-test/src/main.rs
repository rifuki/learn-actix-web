use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let address: String = format!("{}:{}", "0.0.0.0", 80);
    let listener: TcpListener = TcpListener::bind(address)?;

    aw14_integration_test::server::start(listener)?.await?;

    Ok(())
}
