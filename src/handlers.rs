use actix_web::{HttpResponse, Responder, get};

#[get("/hello")]
pub async fn hello() -> impl Responder {
    "Hello Rustaceans!"
}

pub async fn hello_again() -> HttpResponse {
    HttpResponse::Ok().body("Hello again!")
}
