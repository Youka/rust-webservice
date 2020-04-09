mod config;
mod services;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Connect to database
    let conn = rusqlite::Connection::open(config::DB_PATH).expect(&format!("Open SQLite in file '{}' failed?!", config::DB_PATH));
    conn.close().expect("Closing SQLite connection failed?!");
    // Start http server
    actix_web::HttpServer::new(||
        actix_web::App::new()
            .configure(config::register_services)
    )
    .bind("127.0.0.1:".to_string() + config::PORT)?
    .run().await
}