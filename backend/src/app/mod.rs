use actix_web::{HttpServer, App, 
    middleware::{Compress, Logger}};

pub async fn run() -> Result<(), Box<dyn std::error::Error>>{
    log::info!("Booting application...");

    HttpServer::new(move || {
        App::new()
            .wrap(Compress::default())
            .wrap(Logger::default())
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await?;
    
    Ok(())
}