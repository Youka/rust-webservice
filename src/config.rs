#[cfg(windows)]
pub const DB_PATH: &str = concat!(env!("USERPROFILE"), "\\Desktop\\services.db");
#[cfg(linux)]
pub const DB_PATH: &str = concat!(env!("HOME"), "/Desktop/services.db");

pub const PORT: &str = "8081";

pub fn register_services(cfg: &mut actix_web::web::ServiceConfig) {
    use super::services::*;
    cfg.service(info)
        .service(register)
        .service(unregister)
        .service(login)
        .service(logout);
}