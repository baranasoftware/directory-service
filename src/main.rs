mod handlers;
mod models;
mod state;

use crate::handlers::{echo, health, hello};
use crate::state::AppState;
use actix_web::{web, App, HttpServer};
use std::io;
use std::sync::Mutex;

#[actix_web::main]
async fn main() -> io::Result<()> {
    let shared_data = web::Data::new(AppState {
        health_check_response: "app is doing good!".to_string(),
        visit_count: Mutex::new(0),
    });

    let app = move || {
        App::new()
            .app_data(shared_data.clone())
            .service(hello)
            .service(echo)
            .route("/health", web::get().to(health))
    };

    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}
