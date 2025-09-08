use actix_web::{web, App, HttpResponse, HttpServer, Responder};

pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("RustyGate is running 🚀")
}


#[actix_web::main]
async fn main() -> Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(health_check()))  
    })
    .bind("0.0.0.8080")?
    .run()
    .await
}
