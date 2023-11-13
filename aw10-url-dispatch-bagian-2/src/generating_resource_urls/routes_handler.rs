use actix_web::{web, guard, HttpResponse, get, HttpRequest, Result, http::header::LOCATION};

#[get("/service_trigger")]
async fn service_trigger(req: HttpRequest) -> Result<HttpResponse> {
    let url = req.url_for("foo", &["1", "2", "3"])?;

    Ok(
        HttpResponse::Found()
            .insert_header((LOCATION, url.as_str()))
            .finish()
    )
}

pub fn generating_resource_urls(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/services_destination/{a}/{b}/{c}")
            .name("foo")
            .guard(guard::Get())
            .to(|path: web::Path<(u8, u8, u8)>| async {
                let (a, b, c) = path.into_inner();
                HttpResponse::Ok().body(
                    format!("a: {}, b: {}, c: {}", a, b, c)
                )
            })
    );

    cfg.service(service_trigger);
}