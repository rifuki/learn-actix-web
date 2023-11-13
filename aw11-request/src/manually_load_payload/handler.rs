use actix_web::{
    Result, 
    HttpResponse, 
    Error, 
    post, 
    web,
    error
};
use futures::StreamExt;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct PayloadInfo {
    username: String,
    login_count: u32
}

const MAX_SIZE: usize = 262_144; /* <- 256Kb */

#[post("/manual_load_payload")]
pub async fn manual_index(mut payload: web::Payload) -> Result<HttpResponse, Error> {
    let mut body = web::BytesMut::new();

    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;

        if(body.len() + chunk.len()) > MAX_SIZE {
            return Err(error::ErrorBadRequest("overflow"))
        }
        body.extend_from_slice(&chunk);
        println!("chunk: {:#?}", chunk);
    }

    let obj: PayloadInfo = serde_json::from_slice(&body)?;
    println!("obj: {:#?}", obj);

    Ok(
        HttpResponse::Ok().json(obj)
    )
}