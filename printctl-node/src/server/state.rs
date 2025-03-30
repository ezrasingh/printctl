use crate::prelude::*;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use super::devices::{DeviceInfo, DeviceManager, DevicePort};
use crate::discovery;
use crate::discovery::{Idle, ServiceDiscovery};

pub type SharedState<T> = Arc<Mutex<T>>;

pub struct ServerState<T = Idle> {
    node: discovery::Node<T>,
    connections: HashMap<DeviceInfo, DevicePort>,
}

impl<T> ServerState<T> {
    pub fn new(node: discovery::Node<T>) -> Self {
        Self {
            node,
            connections: HashMap::default(),
        }
    }

    pub fn as_arc_mutex(self) -> SharedState<Self> {
        Arc::new(Mutex::new(self))
    }
}

impl DeviceManager for ServerState<ServiceDiscovery> {}
