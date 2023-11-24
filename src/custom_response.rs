use actix_web::{Responder, StatusCode, get};
use std::time::Duration;

#[get("/customresponse")]
pub async fn custom_response() -> impl Responder {
    "This is forbidden!"
            .customize()
            .with_status(StatusCode::FORBIDDEN)
            .insert_header(("X-Forwarded-For", "0.0.0.0"));
}


#[get("/cookies")]
pub async fn cookies() -> impl Responder {

    let cookie1 = Cookie::new("session_cookie", "value");

    let cookie2 = Cookie::build("cookie", "value")
                    .path("/")
                    .secure(true)
                    .domain("example.com")
                    .expires(Duration::seconds(180))
                    .finish();

    HttpResponse::build()
        .cookie(cookie1)
        .cookie(cookie2)
        .finish()
}