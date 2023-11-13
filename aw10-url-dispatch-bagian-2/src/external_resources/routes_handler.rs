use actix_web::{web, HttpRequest, Responder, Result, get};

const YOUTUBE_URL: &str = "https://youtube.com/watch/";
const VIDEO_ID: &str = "oHg5SJYRHA0";

#[get("")]
pub async fn index(req: HttpRequest) -> Result<impl Responder> {
    let url = req.url_for("youtube",  [VIDEO_ID])?;
    assert_eq!(url.as_str(), format!("{}{}", YOUTUBE_URL, VIDEO_ID));

    Ok(url.to_string())
}

pub fn scoped_external_resource(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/external")
            .service(index)
    );

    cfg.external_resource("youtube", "https://youtube.com/watch/{video_id}");
}