use actix_web::HttpResponse;

pub async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hello world!")
}

pub async fn not_found() -> HttpResponse {
    HttpResponse::NotFound().body("Route not found")
}