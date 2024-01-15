use actix_web::rt;
use std::{sync::mpsc, thread};

use crate::server::get_server;

mod server;

fn main() {
    let (tx, _rx) = mpsc::channel();

    // Launch the server in a separate thread
    let handle = thread::spawn(move || {
        let server_future = get_server(tx);
        rt::System::new().block_on(server_future)
    });

    let _ = handle.join().unwrap();
}
