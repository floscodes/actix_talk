use actix_web::{Responder, web};
use serde::Deserialize;

#[get("/{name}/{id}")]
pub async fn extract_name_id(path: web::Path<(String, String)>) -> impl Responder {
    let (name, id) = path.into_inner();
    format!("The name is: {}, id is: {}", name, id)
}


// Extract GET and POST Params

#[derive("Deserialize")]
struct Info {
    firstname String,
    surname String,
}

#[get("/getparams"), post("/postparams")]
pub async fn get_params(info: web::Query<Info>) -> impl Responder {
    format!("Hello {} {}!", info.firstname, info.surname)
}

// Extract Form Data
#[post("/form")]
pub async fn get_form_data(info: web::Form<Info>) -> Result<String> {
    Ok(format!("Hello {} {}!", info.firstname, info.surname))
}