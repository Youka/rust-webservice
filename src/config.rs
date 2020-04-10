#[cfg(windows)]
pub const DB_PATH: &str = concat!(env!("USERPROFILE"), "\\Desktop\\services.db");
#[cfg(linux)]
pub const DB_PATH: &str = concat!(env!("HOME"), "/Desktop/services.db");

pub const PORT: &str = "8081";

pub fn register_services(cfg: &mut actix_web::web::ServiceConfig) {
    use super::services::*;
    cfg.service(info)
        .service(login)
        .service(logout)
        .service(register)
        .service(unregister);
}

pub const IDENTITY_KEY: [u8; 32] = [
    165, 61, 208, 175, 151, 226, 75, 79,
    174, 154, 105, 156, 45, 142, 148, 220,
    223, 6, 239, 112, 53, 53, 54, 51,
    98, 199, 153, 100, 76, 242, 84, 123
];