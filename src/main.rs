use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server");
    HttpServer::new(|| App::new().service(hello))
        .bind("127.0.0.1:8088")?
        // .bind(("0.0.0.0", 4000))?
        .run()
        .await
}
