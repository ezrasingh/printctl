use crate::prelude::*;

use std::sync::Mutex;
use tokio_serial::SerialStream;

use super::devices::DeviceManager;
use crate::discovery;
use crate::discovery::ServiceDiscovery;

pub type SharedState<T> = Mutex<T>;

pub struct ServerState<T> {
    discovery_node: discovery::Node<T>,
}

impl<T> ServerState<T> {
    pub fn new(discovery_node: discovery::Node<T>) -> Self {
        Self { discovery_node }
    }

    pub fn as_mutex(self) -> SharedState<Self> {
        Mutex::new(self)
    }
}

impl ServerState<ServiceDiscovery> {
    pub fn create_stream(&mut self, device_port: &str, baud_rate: u32) -> Result<SerialStream> {
        let port = Self::port_builder(device_port, baud_rate);
        let stream = Self::open_port(&port)?;
        Ok(stream)
    }
}

impl DeviceManager for ServerState<ServiceDiscovery> {}
impl DeviceManager for SharedState<ServerState<ServiceDiscovery>> {}
