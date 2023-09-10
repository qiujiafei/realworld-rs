use actix_web::{get, middleware::Logger, post, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use env_logger::Env;
use std::env;

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
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    let app_addr = env::var(envkey::APP_ADDR).expect("APP Address Not Set");
    println!("Server Start At http://{}", app_addr);
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(hello)
            .service(echo)
            .route("hey", web::get().to(manual_hello))
    })
    .bind(app_addr)?
    .run()
    .await
}
