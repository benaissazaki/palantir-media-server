use std::env;

use actix_cors::Cors;
use actix_files::Files;
use actix_web::{
    middleware,
    web::{self},
    App, HttpServer,
};

use crate::react_app::media_route;

mod app_settings;
mod media_scanner;
mod media_server;
mod react_app;

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
    // Set the exe's parent directory to be the working directory
    if let Ok(exe_path) = env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            if let Err(err) = env::set_current_dir(&exe_dir) {
                eprintln!("Failed to set current directory: {}", err);
            }
        }
    }

    let (host, port) = get_host_and_port();
    #[cfg(debug_assertions)]
    let app_path = "./client/dist";

    #[cfg(not(debug_assertions))]
    let app_path = ".";

    let server = HttpServer::new(move || {
        App::new()
            .wrap(Cors::default().allow_any_origin().send_wildcard())
            .wrap(middleware::Logger::default())
            .service(
                web::scope("/api")
                    .configure(app_settings::init_routes)
                    .configure(media_scanner::init_routes)
                    .configure(media_server::init_routes),
            )
            .service(media_route)
            .service(Files::new("/", app_path).index_file("index.html"))
    })
    .bind((host.clone(), port))?
    .run();

    println!("Server started at http://{}:{}", host, port);

    server.await
}
