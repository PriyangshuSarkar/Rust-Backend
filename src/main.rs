use actix_web::{App, HttpServer};

mod routes;
use routes::*;



#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Move App configuration inside the closure
    let server = HttpServer::new(|| {
        App::new()
            // Add your routes and middleware here
            .service(home)
            .service(hello_user)
            .service(create_new_user)
    })
    .bind(("localhost", 8000))?
    .run();

    println!("Server is running at http://localhost:8000");

    server.await
}
