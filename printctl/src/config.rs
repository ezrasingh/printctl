use serde::Deserialize;

use crate::dashboard;
use printctl_node::server;

#[derive(Debug, Default, Deserialize)]
pub struct DiscoveryConfig {
    name: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
pub struct PrintctlConfig {
    pub discovery: Option<DiscoveryConfig>,
    pub server: Option<server::ServerConfig>,
    pub ui: Option<dashboard::DashboardConfig>,
}
