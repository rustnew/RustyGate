// src/proxy_handler.rs
use actix_web::{web, HttpRequest, HttpResponse, Result as ActixResult};
use std::str::FromStr;
use reqwest::Client;
use std::sync::Arc;
use std::time::Instant;
use crate::config::AppConfig;
use tracing::{info, error};


pub async fn proxy(
    req: HttpRequest,
    path: web::Path<String>,
    body: web::Bytes,
    config: web::Data<Arc<AppConfig>>,
) -> ActixResult<HttpResponse> {
    let start = Instant::now();
    let path_str = format!("/{}", path);

    info!(method = %req.method(), path = %path_str, "Requête reçue");

    // 1. Trouver la route correspondante
    let route = match config.routes.iter().find(|r| path_str.starts_with(&r.path)) {
        Some(r) => r,
        None => {
            info!(path = %path_str, "Route non trouvée");
            return Err(actix_web::error::ErrorNotFound("Route not found"));
        }
    };

    // 2. Construire l'URL cible
    let target_url = format!("{}{}", route.backend, &path_str[route.path.len()..]);
    info!(target_url = %target_url, "Forward vers backend");

    // 3. Forward la requête
    let client = Client::new();
    let mut builder = client
        .request(reqwest::Method::from_str(req.method().as_str()).unwrap(), &target_url)
        .timeout(std::time::Duration::from_secs(5));

    // Copier les headers (sauf Host et Content-Length)
    for (key, value) in req.headers().iter() {
        let key_str = key.as_str().to_lowercase();
        if key_str != "host" && key_str != "content-length" {
            if let Ok(header_value) = value.to_str() {
                builder = builder.header(key.as_str(), header_value);
            }
        }
    }

    // Ajouter le body si présent
    if !body.is_empty() {
        builder = builder.body(body.to_vec());
    }

    // 4. Exécuter la requête
    let res = match builder.send().await {
        Ok(res) => res,
        Err(e) => {
            error!(error = %e, "Backend injoignable");
            return Ok(HttpResponse::BadGateway()
                .insert_header(("X-RustyGate-Error", "backend_unreachable"))
                .body("Backend unreachable"));
        }
    };

    // 5. Construire la réponse
    let status = res.status();
    let headers = res.headers().clone();
    let body_bytes = res.bytes().await.unwrap_or_default();
    let duration = start.elapsed().as_millis();

    info!(status = %status.as_u16(), duration_ms = %duration, "Réponse envoyée");

    let mut response_builder = HttpResponse::build(status);
    response_builder.insert_header(("X-RustyGate-Backend", route.backend.clone()));
    response_builder.insert_header(("X-RustyGate-Duration-ms", duration.to_string()));

    for (key, value) in headers.iter() {
        if let Ok(value_str) = value.to_str() {
            response_builder.append_header((key.as_str(), value_str));
        }
    }

    Ok(response_builder.body(body_bytes))
}