use actix_web::{web, Result, HttpResponse, post};
use futures::StreamExt;
use flate2::read::GzDecoder;
use std::io::Read;

#[post("/content_encoding")]
pub async fn index(mut payload: web::Payload) -> Result<HttpResponse> {
    let mut bytes = web::BytesMut::new();

    while let Some(chunk) = payload.next().await {
        bytes.extend_from_slice(&chunk?);
    }
    
    println!("Payload: {:#?}", bytes);
    let mut decoder = GzDecoder::new(&bytes[..]);
    let mut decoded = String::new();
    println!("decoded: {}", decoded);
    decoder.read_to_string(&mut decoded)?;

    Ok(
        HttpResponse::Ok()
            .body(decoded)
    )
}