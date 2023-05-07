// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use actix_cors::Cors;
use actix_web::{get, middleware::Logger, post, web, App, HttpResponse, HttpServer};
use serde::Deserialize;

#[get("/")]
async fn hello() -> HttpResponse {
    HttpResponse::Ok().body("Hello world!")
}
#[derive(Deserialize, Debug)]
pub struct LoginInfo {
    username: String,
    password: String,
}
#[post("/api/login")]
async fn login(body: web::Json<LoginInfo>) -> HttpResponse {
    let username = "sa".to_string();
    let password = "Sqlserver-2017".to_string();
    if body.username == username && body.password == password {
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::Unauthorized().finish()
    }
}
fn main() {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    tauri::Builder::default()
        .setup(|_app| {
            tauri::async_runtime::spawn(
                HttpServer::new(|| {
                    let cors = Cors::permissive();
                    // .allowed_origin("*")
                    // .allowed_methods(vec!["GET", "POST"])
                    // .allowed_headers(vec![
                    //     http::header::AUTHORIZATION,
                    //     http::header::ACCEPT,
                    //     http::header::CONTENT_TYPE,
                    // ])
                    // .max_age(3600);

                    App::new()
                        .wrap(cors)
                        .wrap(Logger::default())
                        .service(hello)
                        .service(login)
                })
                .bind(("127.0.0.1", 8080))?
                .run(),
            );
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
