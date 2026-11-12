use std::{sync::mpsc::{self, RecvError}, thread};

use actix_files::Files;
use actix_web::{
    dev::ServerHandle,
    middleware, rt,
    web::{self},
    App, HttpServer,
};

use crate::server::react_app::media_route;

pub(crate) mod app_settings;
mod media_scanner;
mod media_server;
mod react_app;

pub async fn get_server(
    tx: mpsc::Sender<ServerHandle>,
    host: String,
    port: u16,
) -> std::io::Result<()> {
    #[cfg(debug_assertions)]
    let app_path = "./client/dist";

    #[cfg(not(debug_assertions))]
    let app_path = "./client";

    let server = HttpServer::new(move || {
        App::new()
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

    let _ = tx.send(server.handle());
    server.await
}

pub fn launch_server(host: String, port: u16) -> Result<ServerHandle, RecvError> {
    let (tx, rx) = mpsc::channel();

    // Launch the server in a separate thread
    thread::spawn(move || {
        let server_future = get_server(tx, host, port);
        rt::System::new().block_on(server_future)
    });

    rx.recv()
}
