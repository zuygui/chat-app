use actix_web::{HttpServer, App, 
    middleware::{Compress, Logger}};

mod handlers;
mod services;
mod utils;

pub async fn run() -> Result<(), Box<dyn std::error::Error>>{
    log::info!("Booting application...");

    HttpServer::new(move || {
        App::new()
            .wrap(Compress::default())
            .wrap(Logger::default())
            .configure(services::setup_data)
            .configure(|c: &mut actix_web::web::ServiceConfig| {
                services::routing::register_urls(c);
            })
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await?;
    
    Ok(())
}