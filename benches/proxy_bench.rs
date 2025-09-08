// benches/proxy_bench.rs
use criterion::{criterion_group, criterion_main, Criterion};
use actix_web::{test, App, HttpResponse, Responder};
use std::sync::Arc;
use rustygate::config::AppConfig;
use rustygate::server;

// Backend mock ultra-simple
async fn mock_backend() -> impl Responder {
    HttpResponse::Ok().body("Hello from mock backend!")
}

// Fonction à benchmarker
async fn bench_proxy_request(app: &mut actix_web::dev::Service<
    actix_web::dev::ServiceRequest,
    actix_web::dev::ServiceResponse,
>) {
    let req = test::TestRequest::get().uri("/api/test").to_request();
    let _resp = test::call_service(app, req).await;
}

// Setup : lance un backend mock + RustyGate en mémoire
fn setup_bench() -> actix_web::dev::Service<
    actix_web::dev::ServiceRequest,
    actix_web::dev::ServiceResponse,
> {
    // 1. Lancer un backend mock sur un port aléatoire
    let backend = actix_web::HttpServer::new(|| {
        actix_web::App::new().route("/{tail:.*}", actix_web::web::get().to(mock_backend))
    })
    .disable_signals()
    .bind("127.0.0.1:0")
    .unwrap();

    let backend_addr = backend.addrs().first().unwrap().clone();
    let backend_handle = backend.run();
    tokio::spawn(backend_handle);

    // Attendre que le backend soit prêt
    std::thread::sleep(std::time::Duration::from_millis(100));

    // 2. Configurer RustyGate pour forwarder vers ce backend
    let config = Arc::new(AppConfig {
        routes: vec![rustygate::config::RouteConfig {
            path: "/api/test".to_string(),
            backend: format!("http://{}", backend_addr),
        }],
    });

    // 3. Créer l'application Actix Web (sans lancer le serveur TCP)
    let app = test::init_service(server::create_app(config));

    app
}

// Benchmark : mesurer le temps d'une requête proxy
fn criterion_benchmark(c: &mut Criterion) {
    let mut rt = tokio::runtime::Runtime::new().unwrap();

    // Setup async
    let mut app = rt.block_on(async { setup_bench() });

    c.bench_function("proxy_single_request", |b| {
        b.to_async(&mut rt).iter(|| async {
            bench_proxy_request(&mut app).await;
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);