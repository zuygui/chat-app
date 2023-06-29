use actix_web::web;
use crate::app::utils::errors::json_error_handler;

pub mod routing;

pub fn setup_data(cfg: &mut web::ServiceConfig) {
    cfg.app_data(web::JsonConfig::default().error_handler(json_error_handler));
}