use std::time::Instant;
use actix::Addr;
use actix_web::{HttpResponse, HttpRequest, web, Error};
use crate::app::utils::chat_server as server;
use actix_web_actors::ws;
use crate::app::utils::websocket::WebSocketSession;

pub async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hello world!")
}

pub async fn not_found() -> HttpResponse {
    HttpResponse::NotFound().body("Route not found")
}

// Actix doc for websocket: https://actix.rs/docs/websockets
pub async fn websocket(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<server::ChatServer>>
) -> Result<HttpResponse, Error>{
    ws::start(
        WebSocketSession {
            id: 0,
            hb: Instant::now(),
            name: None,
            addr: srv.get_ref().clone(),
        },
        &req,
        stream
    )
}