use actix_web::{get, web, Result};

#[derive(serde::Deserialize)]
pub struct JsonInfo {
    username: String,
    password: String,
    role: Option<String>
}

#[get("/v1")]
pub async fn json1(json: web::Json<JsonInfo>) -> Result<String> {
    let result = if let Some(role) = &json.role {
        format!(
            "hi {}, Welcome to json V1: {} and your password: {}", 
            role, 
            json.username, 
            json.password
        )
    } else {
        format!(
            "Welcome to json V1: {} and your password: {}", 
            json.username, 
            json.password
        )
    };

    Ok(result)
}

pub async fn json2(json: web::Json<JsonInfo>) -> Result<String> {
    let result = if let Some(role) = &json.role {
        format!(
            "hi {}, Welcome to json V2 with limitation: {} and your password: {}",
            role,
            json.username,
            json.password
        )
    } else {
        format!(
            "Welcome to json V2 with limitation: {} and your password: {}",
            json.username, json.password
        )
    };

    Ok(result)
}

