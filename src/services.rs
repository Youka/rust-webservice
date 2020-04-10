use actix_web::{get, post, Responder, HttpResponse};
use actix_identity::Identity;

#[get("/")]
async fn info(id: Identity) -> impl Responder {
    if let Some(user) = id.identity() {
        HttpResponse::Ok().json(user)
    } else {
        HttpResponse::Forbidden().finish()
    }
}

#[get("/login")]
async fn login(id: Identity) -> impl Responder {
    let user = "Test;roles=admin,user";
    id.remember(user.to_string());
    HttpResponse::Ok().body("Login: ".to_string() + user)
}

#[get("/logout")]
async fn logout(id: Identity) -> impl Responder {
    id.forget();
    HttpResponse::Ok().body("Logout!")
}

#[post("/register")]
async fn register() -> impl Responder {
    HttpResponse::NotImplemented().body("TODO: register")
}

#[post("/unregister")]
async fn unregister() -> impl Responder {
    HttpResponse::NotImplemented().body("TODO: unregister")
}