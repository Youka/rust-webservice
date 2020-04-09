mod config;
mod services;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Connect to database
    let data = actix_web::web::Data::new(std::sync::Mutex::new(
        rusqlite::Connection::open(config::DB_PATH).expect(&format!("Open SQLite in file '{}' failed?!", config::DB_PATH))
    ));
    // Start http server
    actix_web::HttpServer::new(move ||
        actix_web::App::new()
            .app_data(data.clone())
            .configure(config::register_services)
    )
    .bind("127.0.0.1:".to_string() + config::PORT)?
    .run().await
}