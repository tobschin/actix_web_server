use actix_web::{App, get, HttpResponse, HttpServer, post, Responder, web};

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(echo)
            .route("/hey", web::get().to(manual_hello)) //bsp fuer manuellen handler (ohne #post[("/")])
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}


#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}


#[cfg(test)]
mod tests {
    use actix_web::{test, web, App, http::StatusCode};
    use super::*;

    #[actix_web::test]
    async fn test_manual_hello_response() {
        let app = test::init_service(
            App::new()
                .service(web::resource("/hey").to(manual_hello))
        ).await;

        // Create request object
        let req = test::TestRequest::with_uri("/hey").to_request();

        // Call application
        let resp = test::call_service(&app , req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_index_response() {
        let app = test::init_service(
            App::new()
                .service(index)
        ).await;

        // Create request object
        let req = test::TestRequest::with_uri("/").to_request();

        // Call application
        let resp = test::call_service(&app , req).await;
        println!("{:?}", resp);
        assert_eq!(resp.status(), StatusCode::OK);
        assert!(resp.status().is_success());
    }

}
