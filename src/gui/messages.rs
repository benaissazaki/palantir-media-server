#[derive(Debug, Clone)]
pub enum ServerControlMessage {
    StartServerPressed,
    StopServerPressed,
    HostChanged(String),
    PortChanged(String)
}