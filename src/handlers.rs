use actix_web::{get, post, web, HttpResponse, Responder};
use crate::state::AppState;

pub async fn health(app_state: web::Data<AppState>) -> HttpResponse {
    let health_check_response = &app_state.health_check_response;
    match app_state.visit_count.lock() {
        Ok(v) => {
            let mut visit_count = v;
            let response = format!("{} {} times", health_check_response, visit_count);
            *visit_count += 1;
            HttpResponse::Ok().json(&response)
        },
        Err(e) => {
            HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello from directory service!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
