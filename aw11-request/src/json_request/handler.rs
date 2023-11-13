use actix_web::{Result, web};

#[derive(serde::Deserialize)]
pub struct JsonInfo {
    username: String,
    login_count: u32
}

pub async fn index(payload: web::Json<JsonInfo>) -> Result<String> {
    Ok(
        format!(
            "Welcome {}! and login count: {}", 
            payload.username,
            payload.login_count
        )
    )
}