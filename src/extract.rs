use actix_web::{Responder, HttpRequest};

pub async fn extract_get_params(req: HttpRequest) -> impl Responder {
    format!("The name is: ")
}