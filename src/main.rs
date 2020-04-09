use actix_web::{HttpServer, App, get, Responder, HttpResponse};

// Web routes (=services)
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/again")]
async fn hello_again() -> impl Responder {
    HttpResponse::Ok().body("Hello world again!")
}


// Application configuration
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(hello_again)
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
}