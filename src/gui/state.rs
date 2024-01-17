use actix_web::dev::ServerHandle;

pub struct AppState {
    pub(crate) server_handle: Option<ServerHandle>,
    pub(crate) host: String,
    pub(crate) port: String
}