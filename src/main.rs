use actix_web::{HttpServer, App, web};
use std::io::Error;

mod handlers;
use handlers::*;

mod extract;
use extract::*;

mod custom_response;
use custom_response::*;

#[actix_web::main]
async fn main() -> Result<(), Error> {
    // create a server
    HttpServer::new(|| {
        // create an app with paths and handler functions
        App::new()

            // register "/" path
            .route("/", web::to(|| async {"index"}))

            //register other routes
            .service(hello)
            .route("/helloagain", web::get().to(hello_again))
            .service(web::redirect("/hi", "/hello"))

            // defining a scope (the registered routes will share a common path prefix)
            .service(
                web::scope("/scope") 
                            .route("/sub1", web::to(|| async {"sub1"}))
                            .route("/sub2", web::to(|| async {"sub2"}))
            )

            // extraction demonstration
            .service(extract_name_id)
            .service(get_params)
            .service(get_form_data)

            // customized HttpResponse
            .service(custom_response)
            .service(cookies)
    })
    

    .bind(("0.0.0.0", 8000)).unwrap()
    // .bind(("0.0.0.0", 8000)).unwrap()
    .run().await
}