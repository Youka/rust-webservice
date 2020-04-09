use actix_web::{get, post, Responder, HttpResponse};

#[get("/")]
async fn info() -> impl Responder {
    HttpResponse::Ok().body("TODO: info")
}

#[post("/register")]
async fn register() -> impl Responder {
    HttpResponse::Ok().body("TODO: register")
}

#[post("/unregister")]
async fn unregister() -> impl Responder {
    HttpResponse::Ok().body("TODO: unregister")
}

#[post("/login")]
async fn login() -> impl Responder {
    HttpResponse::Ok().body("TODO: login")
}

#[post("/logout")]
async fn logout() -> impl Responder {
    HttpResponse::Ok().body("TODO: logout")
}