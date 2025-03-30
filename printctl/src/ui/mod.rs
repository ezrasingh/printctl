pub mod tui;
pub mod web;

use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
pub struct UiConfig {
    pub use_web: bool,
    pub http_addr: Option<std::net::IpAddr>,
    pub http_port: Option<u16>,
}
