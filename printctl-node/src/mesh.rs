use crate::prelude::*;

use socket2::{Domain, Protocol, Socket, Type};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use simple_mdns::InstanceInformation;
use simple_mdns::async_discovery::ServiceDiscovery;

#[derive(Default, Clone, Copy)]
pub struct Idle;

pub struct Node<T = Idle> {
    name: String,
    service_addr: IpAddr,
    discovery: T,
}

impl<T> Node<T> {
    const MDNS_SERVICE_NAME: &str = "_printctl._tcp.local";
    const MDNS_SERVICE_PORT: u16 = 8090;
    const MDNS_DEFAULT_TTL: u32 = 60;

    pub fn new(name: impl Into<String>, service_addr: &Option<IpAddr>) -> Result<Node<Idle>> {
        let node = Node {
            name: name.into(),
            service_addr: service_addr.unwrap_or(Self::get_local_address()?),
            discovery: Idle,
        };
        Ok(node)
    }

    /// retrieves the local IP address of the machine by creating a UDP socket.
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
    pub fn start_discovery(self) -> Node<ServiceDiscovery> {
        let instance = InstanceInformation::new(self.name.clone())
            //.with_ip_address(self.service_addr)
            //.with_port(Self::MDNS_SERVICE_PORT)
            .with_attribute(
                env!("CARGO_PKG_NAME").to_string(),
                Some(env!("CARGO_PKG_VERSION").to_string()),
            );

        Node {
            name: self.name,
            service_addr: self.service_addr,
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
    pub fn ip_addresses(&self) -> impl Iterator<Item = IpAddr> {
        self.service().ip_addresses.clone().into_iter()
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
            .map(|instance| Peer::from(instance.clone()))
    }

    pub async fn stop_discovery(mut self) -> Node<Idle> {
        // Removing service from discovery
        self.discovery.remove_service_from_discovery().await;

        Node {
            name: self.name,
            service_addr: self.service_addr,
            discovery: Idle,
        }
    }
}
