pub mod tui;
pub mod web;

use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
pub struct DashboardConfig {
    use_web: bool,
    http_addr: Option<std::net::IpAddr>,
    http_port: Option<u16>,
}
