use actix_web::{get, web, Result, HttpRequest};

#[derive(serde::Deserialize)]
pub struct PathInfo {
    user_id: u32,
    username: String
}

#[get("/v1/{user_id}/{username}")]
pub async fn path1(path: web::Path<PathInfo>) -> Result<String> { /* <- Path Struct need Struct */
    Ok(
        format!(
            "Welcome to path V1: {}!, user_id: {}",
            path.username, path.user_id
        )
    )
}

#[get("/v2/{tuple_0}/{tuple_1}")] /* <- with Path tuple(web::Path<()>) it doesn't care about the name */
pub async fn path2(path: web::Path<(u32, String)>) -> Result<String> {
    let (user_id, username) = path.into_inner(); /* <- must use .into_inner() method to access tuple path */
    Ok(
        format!(
            "Welcome to path V2: {}!, user_id: {}",
            username, user_id
        )
    )
}

#[get("/v3/{user_id}/{username}")]
pub async fn path3(req: HttpRequest) -> Result<String> {
    let user_id: u32 = req.match_info().get("user_id").unwrap().parse().unwrap(); /* <- must set type annotation manually */
    let username: String = req.match_info().query("username").parse().unwrap(); /* <- must set type annotation manually */
    Ok(
        format!(
            "Welcome to path V3: {}!, user_id: {}",
            username, user_id
        )
    )
}