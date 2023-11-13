use actix_web::{HttpRequest, HttpResponse, http::header::CONTENT_TYPE, Responder, HttpServer, App, web};

async fn index(req: HttpRequest) -> HttpResponse {
    if let Some(_) = req.headers().get(CONTENT_TYPE) {
        HttpResponse::Ok().into()
    } else {
        HttpResponse::BadRequest().finish()
    }
}

async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new()
//             .route("/", web::get().to(index))
//             .route("/echo", web::post().to(echo))
//     })
//     .bind(("::", 80))?
//     .run()
//     .await
// }

#[cfg(test)]
mod tests {
    use super::{index, echo};

    use actix_web::{
        test, 
        http::{
            header::ContentType,
            StatusCode
        }, App, web
    };

    #[actix_web::test]
    async fn test_index_ok() {
        let req = test::TestRequest::default()
                .insert_header(ContentType::plaintext())
                .to_http_request();

        let resp = index(req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_index_not_ok() {
        let req = test::TestRequest::default()
            .to_http_request();

        let resp = index(req).await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn test_echo_ok() {
        let app = App::new().route("/echo", web::post().to(echo));
        let mut app = test::init_service(app).await;

        let req_body = "Hello World";
        let req = test::TestRequest::post()
            .uri("/echo")
            .insert_header(ContentType::plaintext())
            .set_payload(req_body)
            .to_request();

        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);

        let resp_body = test::read_body(resp).await;
        assert_eq!(resp_body, req_body);
    }
}
