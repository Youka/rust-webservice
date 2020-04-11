// Dependencies shortcuts
use actix_web::{get, post, put, delete, Responder, HttpResponse, web};
use actix_identity::Identity;
use super::AppData;
use serde::{Deserialize, Serialize};
use rusqlite::params;

// I/O mappings
#[derive(Deserialize)]
struct Login {
    pub name: String,
    pub password: String
}
#[derive(Serialize)]
struct UserInfo {
    pub id: u32,
    pub name: String,
    pub roles: String
}
#[derive(Deserialize)]
struct Registration {
    pub name: String,
    pub password: String
}

// Information services
#[get("/")]
async fn info(id: Identity, data: AppData) -> impl Responder {
    if let Some(user) = id.identity() {
        data.lock().expect("Couldn't lock application data!")
            .query_row("SELECT id, roles FROM users WHERE name=?", params![user], |row|
                Ok((row.get_unwrap(0), row.get_unwrap(1)))
            )
            .map(|id_roles|
                HttpResponse::Ok().json(UserInfo {
                    id: id_roles.0,
                    name: user,
                    roles: id_roles.1
                })
            )
            .unwrap_or_else(|_|
                HttpResponse::BadRequest().finish()
            )
    } else {
        HttpResponse::Unauthorized().finish()
    }
}

// Login services
#[post("/login")]
async fn login(form: web::Form<Login>, data: AppData, id: Identity) -> impl Responder {
    data.lock().expect("Couldn't lock application data!")
        .query_row("SELECT name FROM users WHERE name=? AND password=?", params![form.name, form.password], |_row| Ok(()) )
        .map(|_| {
            id.remember(form.name.to_string());
            HttpResponse::Accepted().finish()
        })
        .unwrap_or_else(|_|
            HttpResponse::BadRequest().finish()
        )
}
#[post("/logout")]
async fn logout(id: Identity) -> impl Responder {
    if id.identity().is_some() {
        id.forget();
        HttpResponse::Accepted().finish()
    } else {
        HttpResponse::Unauthorized().finish()
    }
}

// Registration services
#[put("/register")]
async fn register(form: web::Form<Registration>, data: AppData) -> impl Responder {
    data.lock().expect("Couldn't lock application data!")
        .execute("INSERT INTO users(name, password) VALUES (?, ?)", params![form.name, form.password])
        .map(|_| {
            println!("Registration success!");
            HttpResponse::Created().finish()
        })
        .unwrap_or_else(|err| {
            eprintln!("Registration failed: {}", err);
            HttpResponse::Conflict().finish()
        })
}
#[delete("/unregister")]
async fn unregister(id: Identity, data: AppData) -> impl Responder {
    if let Some(user) = id.identity() {
        data.lock().expect("Couldn't lock application data!")
            .execute("DELETE FROM users WHERE name=?", params![user])
            .map(|_| {
                println!("Unregistration success!");
                HttpResponse::Accepted().finish()
            })
            .unwrap_or_else(|err| {
                eprintln!("Unregistration failed: {}", err);
                HttpResponse::Conflict().finish()
            })
    } else {
        HttpResponse::Unauthorized().finish()
    }
}