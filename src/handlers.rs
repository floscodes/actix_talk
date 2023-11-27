use actix_web::{Responder, get};


pub async fn hello() -> impl Responder {
    "Hello Rustaceans!"
}

#[get("/helloagain")]
pub async fn hello_again() -> impl Responder {
    "Hello again!"
}
