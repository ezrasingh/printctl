use serde::Deserialize;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    node_name: String,
    grpc_address: IpAddr,
    grpc_port: u16,
}

impl ServerConfig {
    pub fn new(
        node_name: impl Into<String>,
        grpc_address: Option<IpAddr>,
        grpc_port: Option<u16>,
    ) -> Self {
        Self {
            node_name: node_name.into(),
            grpc_address: grpc_address.unwrap_or(Ipv4Addr::new(0, 0, 0, 0).into()),
            grpc_port: grpc_port.unwrap_or(50051),
        }
    }
    pub fn node_name(&self) -> &str {
        &self.node_name
    }

    pub fn grpc_address(&self) -> &IpAddr {
        &self.grpc_address
    }

    pub fn grpc_port(&self) -> &u16 {
        &self.grpc_port
    }

    pub fn grpc_socket(&self) -> SocketAddr {
        SocketAddr::new(*self.grpc_address(), *self.grpc_port())
    }
}
