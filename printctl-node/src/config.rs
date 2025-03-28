use crate::prelude::*;

use std::net::IpAddr;

use super::mesh;

pub struct Config {
    node_name: String,
    service_address: Option<IpAddr>,
    grpc_socket: Option<std::net::SocketAddr>,
}

impl Config {
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

impl TryInto<mesh::Node> for Config {
    type Error = crate::error::Error;

    fn try_into(self) -> Result<mesh::Node> {
        use super::mesh::Idle;

        let node = mesh::Node::<Idle>::new(self.node_name(), self.service_address())?;
        Ok(node)
    }
}
