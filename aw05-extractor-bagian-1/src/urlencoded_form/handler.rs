use actix_web::{web, post, Result};

#[derive(serde::Deserialize)]
pub struct UrlEncodedForm{
    username: String,
    password: String
}

#[post("/v1")]
pub async fn form1(form: web::Form<UrlEncodedForm>) -> Result<String> {
    Ok(
        format!(
            "Welcome to url-encoded form V1 {}! and your password: {}",
            form.username,
            form.password
        )
    )
}

pub async fn form2(form: web::Form<UrlEncodedForm>) -> Result<String> {
    Ok(
        format!(
            "Welcome to url-encoded form V2 with limitation {}! and your password: {}",
            form.username,
            form.password
        )
    )
}