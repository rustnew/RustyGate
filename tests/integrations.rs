// tests/integration.rs
use actix_web::{test, web, App, HttpResponse, HttpServer, Responder};
use std::sync::Arc;
use rustygate::config::AppConfig;
use rustygate::src::server; // On réutilise start_server, mais en mode test

// Backend mock simple
async fn mock_backend() -> impl Responder {
    HttpResponse::Ok().body("Hello from mock backend!")
}

#[actix_web::test]
async fn test_proxy_forwarding() {
    // 1. Créer un backend mock
    let backend = HttpServer::new(|| {
        App::new().route("/{tail:.*}", web::get().to(mock_backend))
    })
    .disable_signals() // Important pour les tests
    .bind("127.0.0.1:0") // Port aléatoire
    .unwrap();

    let backend_addr = backend.addrs().first().unwrap().clone();
    let backend_handle = backend.run();
    
    // Lancer le backend en arrière-plan
    tokio::spawn(backend_handle);

    // Attendre que le backend soit prêt (optionnel, mais prudent)
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // 2. Créer une config de test pointant vers le backend mock
    let config = Arc::new(AppConfig {
        routes: vec![rustygate::config::RouteConfig {
            path: "/api/test".to_string(),
            backend: format!("http://{}", backend_addr),
        }],
    });

    // 3. Lancer RustyGate en mode test
    let mut app = test::init_service(
        server::create_app(config) // ← On va créer cette fonction dans server.rs
    ).await;

    // 4. Envoyer une requête au proxy
    let req = test::TestRequest::get().uri("/api/test").to_request();
    let resp = test::call_service(&mut app, req).await;

    // 5. Vérifier la réponse
    assert!(resp.status().is_success());
    let body = test::read_body(resp).await;
    let body_str = std::str::from_utf8(&body).unwrap();
    assert_eq!(body_str, "Hello from mock backend!");
}