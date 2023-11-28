use actix_web::{HttpServer, App, web, middleware};
use actix_files;
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

            // add middleware and accept trailing slashes sent by the client
            .wrap(middleware::NormalizePath::trim())

            // register routes
            .route("/hello", web::to(hello))
            .service(hello_again)
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
            .service(post_form)

            // customized HttpResponse
            .service(custom_response)
            .service(cookies)

            // serve a static directory
            .service(
                actix_files::Files::new("/static", "./static").show_files_listing()
            )
    })
    

    .bind(("0.0.0.0", 8000)).unwrap()
    // .bind(("0.0.0.0", 8000)).unwrap()
    .run().await
}