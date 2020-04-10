// Dependencies shortcuts
use actix_web::{get, post, Responder, HttpResponse};
use actix_identity::Identity;
use super::AppData;

// Information services
#[get("/")]
async fn info(id: Identity) -> impl Responder {
    if let Some(user) = id.identity() {
        HttpResponse::Ok().json(user)
    } else {
        HttpResponse::Forbidden().finish()
    }
}

// Login services
#[post("/login")]
async fn login(id: Identity, _data: AppData) -> impl Responder {
    let user = "Test;roles=admin,user";
    id.remember(user.to_string());
    HttpResponse::Ok().body("Login: ".to_string() + user)
}
#[post("/logout")]
async fn logout(id: Identity) -> impl Responder {
    id.forget();
    HttpResponse::Ok().body("Logout!")
}

// Registration services
#[post("/register")]
async fn register(_data: AppData) -> impl Responder {
    HttpResponse::NotImplemented().body("TODO: register")
}
#[post("/unregister")]
async fn unregister(_id: Identity, _data: AppData) -> impl Responder {
    HttpResponse::NotImplemented().body("TODO: unregister")
}