use actix_web::{HttpServer, App, web};
use std::io::Error;

mod handlers;
use handlers::*;

mod extract;
use extract::*;

#[actix_web::main]
async fn main() -> Result<(), Error> {
    // create a server
    HttpServer::new(|| {
        // create an app with paths and handler functions
        App::new()

            .service(hello)
            .route("/helloagain", web::get().to(hello_again))
            .service(web::redirect("/", "/hello"))

            // extraction demonstration
            .service(extract_name_id)
            .service(get_form_data)
    })
    

    .bind(("0.0.0.0", 8080)).unwrap()
    // .bind(("0.0.0.0", 8000)).unwrap()
    .run().await
}