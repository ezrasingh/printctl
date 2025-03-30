use crate::prelude::*;

use socket2::{Domain, Protocol, Socket, Type};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use serde::Deserialize;
use simple_mdns::InstanceInformation;
use simple_mdns::async_discovery::ServiceDiscovery;

#[derive(Debug, Deserialize)]
pub struct DiscoveryConfig {
    name: String,
}

impl DiscoveryConfig {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl Default for DiscoveryConfig {
    fn default() -> Self {
        use gethostname::gethostname;
        let name = gethostname()
            .into_string()
            .expect("Could not detect hostname");
        Self { name }
    }
}

#[derive(Default, Clone, Copy)]
pub struct Idle;

#[derive(Default)]
pub struct Node<T = Idle> {
    config: DiscoveryConfig,
    discovery: T,
}

impl<T> Node<T> {
    const MDNS_SERVICE_NAME: &str = "_printctl._tcp.local";
    const MDNS_SERVICE_PORT: u16 = 8090;
    const MDNS_DEFAULT_TTL: u32 = 60;

    pub fn name(&self) -> String {
        self.config.name().to_string()
    }

    /// retrieves the local IP address of the machine by creating a dummy UDP socket.
    /// this method does not actually send any packets; it just determines which
    /// local IP the OS would use to reach an external address.
    pub fn get_local_address() -> Result<IpAddr> {
        // reference: https://github.com/tsirysndr/local-ip-addr/blob/master/src/lib.rs
        // create a new IPv4 UDP (datagram) socket.
        let socket = Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::UDP))?;
        // define a remote IP and port to "connect" to.
        // 10.254.254.254 is a non-routable address, chosen arbitrarily.
        let socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(10, 254, 254, 254)), 1).into();
        // "connect" the socket to this address (no actual data is sent).
        // this step forces the OS to determine which local IP to use.
        socket.connect(&socket_addr)?;
        // retrieve the local address assigned to this socket.
        let socket_addr = socket.local_addr()?;
        // extract host local ipv4
        let socket_addr = socket_addr
            .as_socket()
            .expect("could not find local socket address");
        Ok(socket_addr.ip())
    }
}

impl Node<Idle> {
    pub fn new(config: DiscoveryConfig) -> Self {
        Self {
            config,
            discovery: Idle,
        }
    }

    pub fn start_discovery(self) -> Node<ServiceDiscovery> {
        let instance = InstanceInformation::new(self.name()).with_attribute(
            env!("CARGO_PKG_NAME").to_string(),
            Some(env!("CARGO_PKG_VERSION").to_string()),
        );
        Node {
            config: self.config,
            discovery: ServiceDiscovery::new(
                instance,
                Self::MDNS_SERVICE_NAME,
                Self::MDNS_DEFAULT_TTL,
            )
            .expect("Failed to start service discovery"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Peer(InstanceInformation);

impl Peer {
    pub fn service(&self) -> &InstanceInformation {
        &self.0
    }
    pub fn name(&self) -> String {
        self.service().escaped_instance_name()
    }
    pub fn ip_addresses(&self) -> impl Iterator<Item = &IpAddr> {
        self.service().ip_addresses.iter()
    }
}

impl From<InstanceInformation> for Peer {
    fn from(instance: InstanceInformation) -> Self {
        Self(instance)
    }
}

impl Node<ServiceDiscovery> {
    pub async fn peers(&self) -> impl Iterator<Item = Peer> {
        self.discovery
            .get_known_services()
            .await
            .into_iter()
            .map(|instance| Peer::from(instance))
    }

    pub async fn stop_discovery(mut self) -> Node<Idle> {
        self.discovery.remove_service_from_discovery().await;
        Node {
            config: self.config,
            discovery: Idle,
        }
    }
}
