pub mod string_decoder;

use crate::prelude::*;

use tokio_serial::{SerialPortBuilder, SerialPortInfo, SerialPortType, SerialStream, UsbPortInfo};

use super::api::grpc;

pub type DeviceInfo = SerialPortInfo;

pub trait DeviceManager {
    fn list_ports() -> Vec<DeviceInfo> {
        tokio_serial::available_ports().unwrap_or_default()
    }

    fn port_builder(device_path: &str, baud_rate: u32) -> SerialPortBuilder {
        tokio_serial::new(device_path, baud_rate)
    }

    fn open_port(port_builder: &SerialPortBuilder) -> Result<SerialStream> {
        let stream = SerialStream::open(port_builder)?;
        Ok(stream)
    }
}

impl Into<grpc::v0::DeviceInfo> for DeviceInfo {
    fn into(self) -> grpc::v0::DeviceInfo {
        match self.port_type {
            SerialPortType::UsbPort(UsbPortInfo {
                vid,
                pid,
                serial_number,
                manufacturer,
                product,
            }) => grpc::v0::DeviceInfo {
                port_name: self.port_name,
                vendor_id: vid.into(),
                product_id: pid.into(),
                serial_number: serial_number.unwrap_or_default(),
                manufacturer: manufacturer.unwrap_or_default(),
                product: product.unwrap_or_default(),
            },
            _ => grpc::v0::DeviceInfo {
                port_name: self.port_name,
                ..Default::default()
            },
        }
    }
}
