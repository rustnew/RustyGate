// src/server.rs
use actix_web::{web, App, HttpResponse, HttpServer, Responder, http::Method};
use std::sync::Arc;
use crate::config::AppConfig;
use crate::proxy_handler;

pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("RustyGate is running 🚀")
}

pub async fn start_server(config: Arc<AppConfig>) -> std::io::Result<()> {
    println!("📜 Routes configurées :");
    for route in &config.routes {
        println!("  {} → {}", route.path, route.backend);
    }

    HttpServer::new(move || {
        let app_config = config.clone();
        App::new()
            .app_data(web::Data::new(app_config))
            .route("/", web::get().to(health_check))
            // ✅ UN SEUL .route() — mais qui gère TOUTES les méthodes
            .route(
                "/{tail:.*}",
                web::route()
                    .method(Method::GET)
                    .method(Method::POST)
                    .method(Method::PUT)
                    .method(Method::PATCH)
                    .method(Method::DELETE)
                    .method(Method::HEAD)
                    .method(Method::OPTIONS)
                    .to(proxy_handler::proxy),
            )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}