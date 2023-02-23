use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use std::env;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server");
    let port = env::var("PORT").expect("Missing $PORT env var");
    let address = format!("0.0.0.0:{}", port);
    HttpServer::new(|| App::new().service(hello))
        .bind(address)?
        .run()
        .await
}
