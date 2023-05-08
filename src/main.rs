use actix_web::{App, HttpServer};

mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut args = std::env::args().skip(1);
    /* Get host and port as CLI args or use default values */
    let host = args.next().unwrap_or("127.0.0.1".to_string());
    let port = args
        .next()
        .unwrap_or("8080".to_string())
        .parse::<u16>()
        .unwrap();

    let server = HttpServer::new(|| App::new().service(routes::hello))
        .bind((host.clone(), port))?
        .run();

    println!("Server started at http://{}:{}", host, port);

    server.await
}
