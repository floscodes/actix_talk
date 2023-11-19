use actix_web::{HttpServer, App, web};
use std::io::Error;

mod handlers;
use handlers::*;

mod extract;

#[actix_web::main]
async fn main() -> Result<(), Error> {
    // create a server
    HttpServer::new(|| {
        // create an app with paths and handler functions
        App::new()

            // register paths and link them to a handler-function (function that implements the Handler-trait)
            .service(hello)
            .route("/helloagain", web::get().to(hello_again))
            .service(web::redirect("/", "/hello"))
    })
    

    .bind(("0.0.0.0", 8080)).unwrap()
    // .bind(("0.0.0.0", 8000)).unwrap()
    .run().await
}