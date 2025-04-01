use std::collections::HashMap;
use std::sync::Mutex;

use super::devices::{DeviceInfo, DeviceManager, DevicePort};
use crate::discovery;
use crate::discovery::ServiceDiscovery;

pub type SharedState<T> = Mutex<T>;

pub struct ServerState<T> {
    discovery_node: discovery::Node<T>,
    connections: HashMap<DeviceInfo, DevicePort>,
}

impl<T> ServerState<T> {
    pub fn new(discovery_node: discovery::Node<T>) -> Self {
        Self {
            discovery_node,
            connections: HashMap::default(),
        }
    }

    pub fn as_mutex(self) -> SharedState<Self> {
        Mutex::new(self)
    }
}

impl DeviceManager for ServerState<ServiceDiscovery> {}
