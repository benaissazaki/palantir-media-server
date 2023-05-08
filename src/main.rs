use actix_web::{App, HttpServer};

mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let host = "127.0.0.1";
    let port = 8080;
    let server = HttpServer::new(|| App::new().service(routes::hello))
        .bind((host, port))?
        .run();

    println!("Server started at http://{}:{}", host, port);

    server.await
}
