rustygate/
├── .github/
│   └── workflows/
│       └── ci.yml                  # CI/CD : build, test, release
├── benches/
│   └── proxy_bench.rs              # Benchmark de performance (avec criterion)
├── config/
│   └── default.yaml                # Fichier de configuration par défaut
├── docs/
│   └── ARCHITECTURE.md             # Documentation technique détaillée
├── examples/
│   ├── plugin_add_header.rs        # Exemple de plugin en Rust (à compiler en WASM)
│   └── docker-compose.observability.yml  # Pour lancer Prometheus + Grafana + Jaeger
├── src/
│   ├── main.rs                     # Point d'entrée principal
│   ├── server.rs                   # Configuration et lancement du serveur Actix Web
│   ├── config.rs                   # Parsing, validation et reload de la config
│   ├── proxy_handler.rs            # Logique principale de routage et forward
│   ├── middleware/
│   │   ├── mod.rs
│   │   ├── auth.rs                 # Middleware JWT
│   │   ├── rate_limit.rs           # Middleware rate limiting
│   │   ├── cors.rs                 # Middleware CORS
│   │   └── metrics.rs              # Middleware pour incrémenter les métriques
│   ├── balancer/
│   │   ├── mod.rs
│   │   ├── round_robin.rs          # Stratégie Round Robin
│   │   └── least_conn.rs           # Stratégie Least Connections (optionnel)
│   ├── health.rs                   # Health checker périodique des backends
│   ├── metrics.rs                  # Setup de l'export Prometheus
│   ├── tracing.rs                  # Setup OpenTelemetry + propagation
│   └── plugins/
│       ├── mod.rs
│       ├── manager.rs              # Chargement, gestion, exécution des plugins WASM
│       └── api.rs                  # Interface commune pour les plugins (trait)
├── tests/
│   ├── integration.rs              # Tests bout-en-bout (avec backends mockés)
│   └── middleware/
│       ├── auth.rs
│       └── rate_limit.rs           # Tests unitaires spécifiques
├── Cargo.toml                      # Dépendances, features, metadata
├── Cargo.lock                      # Versions exactes des dépendances
├── Dockerfile                      # Pour construire l'image de production
├── docker-compose.yml              # Pour lancer RustyGate + backends de test
├── README.md                       # Documentation utilisateur / contributeurs
└── .gitignore                      # Ignore target/, logs, binaires, etc.