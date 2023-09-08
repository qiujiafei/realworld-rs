use dotenv::dotenv;
use std::env;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

mod envkey;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("HelloWorld")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey There!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    println!("Hello, world!");
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("hey", web::get().to(manual_hello))
    })
    .bind(env::var(envkey::env_key::APP_ADDR).expect("APP Address Not Set"))?
    .run()
    .await
}
