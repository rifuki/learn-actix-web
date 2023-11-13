use actix_web::{
    Result, 
    web, 
    HttpResponse, 
    get,
    http::header::ContentType
};
use futures::StreamExt;

#[get("/streaming_request")]
pub async fn index(mut payload: web::Payload) -> Result<HttpResponse> {
    let mut bytes= web::BytesMut::new();

    while let Some(chunks) = payload.next().await {
        let chunks = chunks?;
        println!("chunks: {:?}", chunks);
        bytes.extend_from_slice(&chunks);
    }

    let num_bytes = bytes.len();
    let response_body = format!("Received {} bytes", num_bytes);

    Ok(
        HttpResponse::Ok()
            .content_type(ContentType::plaintext())
            .body(response_body)
    )
}