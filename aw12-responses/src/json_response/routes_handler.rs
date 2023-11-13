use actix_web::{web, Result, Responder, get, HttpResponse};

#[derive(serde::Serialize)]
pub struct MyObj {
   pub name: String
}

#[get("/json_response/{name}")]
async fn index(path: web::Path<String>) -> Result<impl Responder> {
    let obj = MyObj {
        name: path.into_inner()
    };

    Ok(
        HttpResponse::Ok()
            .json(obj)
    )
}

pub fn scoped_json_response(cfg: &mut web::ServiceConfig) {
    cfg.service(index);
}
