use actix_web::{get, post, HttpResponse, Responder};

pub async fn health() -> impl Responder {
    HttpResponse::Ok().body("The directory service is up and running!")
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello from directory service!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
