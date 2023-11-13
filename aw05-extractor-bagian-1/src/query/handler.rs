use actix_web::{get, web};

#[derive(serde::Deserialize)]
pub struct QueryInfo {
   username: String,
   id: Option<u32>
}

/* * query use ask notation (?) */
#[get("/query")]
pub async fn query(query: web::Query<QueryInfo>) -> String {
    if let Some(user_id) = query.id {
        format!("Welcome to query: {} and your user_id: {}", query.username, user_id)
    } else {
        format!("Welcome to query: {}", query.username)
    }
}