use serde::Deserialize;
use std::net::IpAddr;

#[derive(Debug, Default, Deserialize)]
pub struct ServerConfig {
    node_name: String,
    service_address: Option<IpAddr>,
    grpc_socket: Option<std::net::SocketAddr>,
}

impl ServerConfig {
    pub fn new(
        node_name: impl Into<String>,
        service_address: Option<IpAddr>,
        grpc_socket: Option<std::net::SocketAddr>,
    ) -> Self {
        Self {
            node_name: node_name.into(),
            service_address,
            grpc_socket,
        }
    }
    pub fn node_name(&self) -> &str {
        &self.node_name
    }

    pub fn service_address(&self) -> &Option<IpAddr> {
        &self.service_address
    }

    pub fn grpc_socket(&self) -> &Option<std::net::SocketAddr> {
        &self.grpc_socket
    }
}
