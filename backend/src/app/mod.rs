use actix::Actor;
use actix_web::{HttpServer, App, web, middleware::{Compress, Logger}};

use std::sync::{atomic::{AtomicUsize},Arc};

mod handlers;
mod services;
mod utils;

use utils::chat_server as server;

pub async fn run() -> Result<(), Box<dyn std::error::Error>>{
    log::info!("Booting application...");

    // set up applications state
    // keep a count of the number of visitors
    let app_state = Arc::new(AtomicUsize::new(0));

    // start chat server actor
    let server = server::ChatServer::new(app_state.clone()).start();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::from(app_state.clone()))
            .app_data(web::Data::new(server.clone()))
            .wrap(Compress::default())
            .wrap(Logger::default())
            .configure(services::setup_data)
            .configure(|c: &mut actix_web::web::ServiceConfig| {
                services::routing::register_urls(c);
            })
    })
    .workers(2)
    .bind("0.0.0.0:8080")?
    .run()
    .await?;
    
    Ok(())
}