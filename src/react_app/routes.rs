use std::path::PathBuf;

use actix_files::NamedFile;
use actix_web::{Responder, get};

#[get("/media/{media_name}")]
pub async fn media_route() -> impl Responder {
  #[cfg(debug_assertions)]
  let app_path = PathBuf::from("./client/dist/index.html");

  #[cfg(not(debug_assertions))]
  let app_path = PathBuf::from("./index.html");

  println!("{}", app_path.to_str().unwrap());

  NamedFile::open(app_path)
}
