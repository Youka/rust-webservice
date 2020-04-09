use actix_web::{get, Responder, HttpResponse};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/again")]
async fn hello_again() -> impl Responder {
    HttpResponse::Ok().body("Hello world again!")
}