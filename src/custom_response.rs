use actix_web::{Responder, HttpResponse, http::StatusCode, cookie::{Cookie, time::Duration}, get};

#[get("/customresponse")]
pub async fn custom_response() -> impl Responder {
    "This is forbidden!"
            .customize()
            .with_status(StatusCode::FORBIDDEN)
            .insert_header(("X-Forwarded-For", "0.0.0.0"))
}


#[get("/cookies")]
pub async fn cookies() -> impl Responder {

    let cookie1 = Cookie::new("first_cookie", "value");

    let cookie2 = Cookie::build("second_cookie", "value")
                    .path("/")
                    .domain("example.com")
                    .max_age(Duration::minutes(3))
                    .finish();

    HttpResponse::build(StatusCode::OK)
        .cookie(cookie1)
        .cookie(cookie2)
        .finish()
}