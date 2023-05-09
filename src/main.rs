use actix_web::{App, HttpServer};

mod routes;

fn get_host_and_port() -> (String, u16) {
    let mut args = std::env::args().skip(1);

    let host = args.next().unwrap_or("127.0.0.1".to_string());
    let port = match args.next() {
        Some(p) => match p.parse::<u16>() {
            Ok(port) => port,
            Err(_) => 8080,
        },
        None => 8080,
    };

    (host, port)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let (host, port) = get_host_and_port();
    let server = HttpServer::new(|| App::new().service(routes::hello))
        .bind((host.clone(), port))?
        .run();

    println!("Server started at http://{}:{}", host, port);

    server.await
}
