// Define submodules
mod config;
mod services;

// Dependencies shortcuts
use std::{sync::Mutex, env, io::Result};
use rusqlite::Connection;
use actix_web::{web, HttpServer, App, middleware};
use actix_identity::{IdentityService, CookieIdentityPolicy};

// Type of application shared data
type AppData = web::Data<Mutex<Connection>>;

// Program entry
#[actix_rt::main]
async fn main() -> Result<()> {
    // Initialize global logging
    env::set_var("RUST_LOG", "info");
    env_logger::init();
    // Connect to database
    let conn = Connection::open(config::DB_PATH).expect(&format!("Open SQLite file '{}' failed?!", config::DB_PATH));
    // Pack application shared data
    let data: AppData = web::Data::new(Mutex::new(conn));
    // Start http server
    HttpServer::new(move ||
        // Extend server by services & filters
        App::new()
            .app_data(data.clone())
            .configure(config::register_services)
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&config::IDENTITY_KEY)
                    .name("identity")
                    .secure(false)
            ))
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
    )
    .bind("127.0.0.1:".to_string() + config::PORT)?
    .keep_alive(3)
    .run().await
}