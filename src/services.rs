// Dependencies shortcuts
use actix_web::{get, post, Responder, HttpResponse, web};
use actix_identity::Identity;
use super::AppData;
use serde::Deserialize;

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
#[derive(Deserialize)]
struct Login {
    pub name: String,
    pub password: String
}
#[post("/login")]
async fn login(_form: web::Form<Login>, _data: AppData, id: Identity) -> impl Responder {
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
#[derive(Deserialize)]
struct Registration {
    pub name: String,
    pub password: String
}
#[post("/register")]
async fn register(_form: web::Form<Registration>, _data: AppData) -> impl Responder {
    HttpResponse::NotImplemented().body("TODO: register")
}
#[post("/unregister")]
async fn unregister(_id: Identity, _data: AppData) -> impl Responder {
    HttpResponse::NotImplemented().body("TODO: unregister")
}