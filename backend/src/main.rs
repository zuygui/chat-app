extern crate log;

mod app;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok().expect("Failed to read .env file");
    pretty_env_logger::init();

    log::info!("Starting server...");

    match app::run().await {
        Ok(_) => std::process::exit(0),
        Err(err) => {
            log::error!("Error: {}", err);
            std::process::exit(1);
        }
    }
}