use dotenv::dotenv;
use lol::managed_accounts::get_managed_accounts;
use lol::sent_transactions::get_sent_transactions;
use std::env;

use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/managed_accounts")]
async fn managed_accounts_handler() -> impl Responder {
    let accounts = get_managed_accounts();
    HttpResponse::Ok().body(format!("{:?}", accounts))
}

#[get("/sent_transactions")]
async fn sent_transactions_handler() -> impl Responder {
    let transactions = get_sent_transactions();
    HttpResponse::Ok().body(format!("{:?}", transactions))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    println!("Starting server");
    let port = env::var("PORT").expect("Missing $PORT env var");
    let address = format!("0.0.0.0:{}", port);
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(managed_accounts_handler)
            .service(sent_transactions_handler)
    })
    .bind(address)?
    .run()
    .await
}
