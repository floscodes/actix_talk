use actix_web::{HttpRequest, Result, Responder, web, get, post};
use serde::Deserialize;

#[get("/{name}/{message}/{id}")]
pub async fn extract_name_id(req: HttpRequest) -> impl Responder {
    let name = req.match_info().query("name");
    let message = req.match_info().query("message");
    let id = req.match_info().query("id");
    format!("The name is: {}, message: {}, id: {}", name, message, id)
}


// Extract GET and POST Params

#[derive(Deserialize)]
struct Info {
    firstname: String,
    surname: String,
}

#[get("/get_params")]
pub async fn get_params(info: web::Query<Info>) -> impl Responder {
    format!("Hello, {} {}!", info.firstname, info.surname)
}


// Extract Form Data
#[post("/post_form")]
pub async fn post_form(info: web::Form<Info>) -> Result<String> {
    Ok(format!("Hello, {} {}!", info.firstname, info.surname))
}

/*
Try out post_form in CLI:

curl -X POST http://localhost:8000/post_form -H "Content-Type: application/x-www-form-urlencoded" -d "firstname=Tom&surname=Miller" 
*/