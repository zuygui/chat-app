use actix_web::web;
use super::views;

pub fn register_urls() -> actix_web::Scope {
    web::scope("")
        // Route /
        .route("/", web::get().to(views::index))
        .default_service(web::route().to(views::not_found))
}