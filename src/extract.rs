use actix_web::{HttpRequest, Result, Responder, web, get, post};
use serde::Deserialize;

#[get("/{name}/{message}/{id}")]
pub async fn extract_name_id(req: HttpRequest) -> impl Responder {
    let name = req.match_info().query("name");
    let message = req.match_info().query("message");
    let id = req.match_info().query("id");
    format!("The name is: {}, id is: {}, message: {}", name, id, message)
}


// Extract GET and POST Params

#[derive(Deserialize)]
struct Info {
    firstname: String,
    surname: String,
}

#[get("/getparams")]
pub async fn get_params(info: web::Query<Info>) -> impl Responder {
    format!("Hello {} {}!", info.firstname, info.surname)
}

// Extract Form Data
#[post("/form")]
pub async fn get_form_data(info: web::Form<Info>) -> Result<String> {
    Ok(format!("Hello {} {}!", info.firstname, info.surname))
}