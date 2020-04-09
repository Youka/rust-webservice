mod services;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Connect to database
    let conn = rusqlite::Connection::open_in_memory().expect("Open SQLite in memory failed?!");
    conn.close().expect("Closing SQLite connection failed?!");
    // Start http server
    actix_web::HttpServer::new(||
        actix_web::App::new()
            .service(services::hello)
            .service(services::hello_again)
    )
    .bind("127.0.0.1:8081")?
    .run().await
}