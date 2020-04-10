mod config;
mod services;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Initialize global logging
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    // Connect to database
    let conn = rusqlite::Connection::open(config::DB_PATH).expect(&format!("Open SQLite file '{}' failed?!", config::DB_PATH));
    // Pack application shared data
    let data = actix_web::web::Data::new(std::sync::Mutex::new(conn));
    // Start http server
    actix_web::HttpServer::new(move ||
        actix_web::App::new()
            .app_data(data.clone())
            .configure(config::register_services)
            .wrap(actix_identity::IdentityService::new(
                actix_identity::CookieIdentityPolicy::new(&config::IDENTITY_KEY)
                    .name("identity")
                    .secure(false)
            ))
            .wrap(actix_web::middleware::Logger::default())
            .wrap(actix_web::middleware::Compress::default())
    )
    .bind("127.0.0.1:".to_string() + config::PORT)?
    .keep_alive(3)
    .run().await
}