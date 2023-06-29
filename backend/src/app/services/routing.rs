use actix_web::web;

use crate::app::handlers;

pub fn register_urls(cfg: &mut web::ServiceConfig) {
    cfg.service(handlers::urls::register_urls());
}