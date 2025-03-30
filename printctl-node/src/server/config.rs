use serde::Deserialize;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    grpc_address: IpAddr,
    grpc_port: u16,
}

impl ServerConfig {
    pub fn new(grpc_address: IpAddr, grpc_port: u16) -> Self {
        Self {
            grpc_address,
            grpc_port,
        }
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

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            grpc_address: IpAddr::V4(Ipv4Addr::UNSPECIFIED),
            grpc_port: 50051,
        }
    }
}
