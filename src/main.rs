// src/main.rs
pub mod config;
pub mod server;
pub mod proxy_handler;

use tracing_subscriber;
use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialiser tracing
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .json() // Logs en JSON
        .init();

    // Charger la configuration
    let config_path = "config/default.yaml";
    let config = match config::AppConfig::load_from_file(config_path) {
        Ok(cfg) => {
            println!("✅ Configuration chargée depuis {}", config_path);
            Arc::new(cfg)
        }
        Err(e) => {
            eprintln!("❌ Erreur de chargement de la config: {}", e);
            std::process::exit(1);
        }
    };

    // Lancer le serveur
    println!("🚀 RustyGate démarré sur http://localhost:8080");
    server::start_server(config).await
}