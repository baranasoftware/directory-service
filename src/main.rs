use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::io;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello from directory service!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn health() -> impl Responder {
    HttpResponse::Ok().body("The directory service is up and running!")
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/health", web::get().to(health))
    })
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
