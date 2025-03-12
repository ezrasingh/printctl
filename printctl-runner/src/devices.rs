use serial::core::SerialDevice;

#[derive(Debug)]
pub struct Device {
    path: String,
    vendor: String,
    product: String,
    usb: String,
}

impl Device {
    pub fn path(&self) -> &String {
        &self.path
    }
    pub fn vendor(&self) -> &String {
        &self.vendor
    }
    pub fn product(&self) -> &String {
        &self.product
    }
    pub fn usb(&self) -> &String {
        &self.usb
    }

    pub fn port(&self) -> Result<impl SerialDevice, serial::Error> {
        serial::open(&self.path)
    }
}

use serial_enumerator::{get_serial_list, SerialInfo};

impl From<SerialInfo> for Device {
    fn from(serial_info: SerialInfo) -> Self {
        let field_or_else = || Some(String::from("--"));
        return Self {
            path: serial_info.name,
            vendor: serial_info.vendor.or_else(field_or_else).unwrap(),
            product: serial_info.product.or_else(field_or_else).unwrap(),
            usb: serial_info
                .usb_info
                .and_then(|usbinfo| Some(format!("{}:{}", usbinfo.vid, usbinfo.pid)))
                .or_else(field_or_else)
                .unwrap(),
        };
    }
}

pub fn list_devices() -> Vec<Device> {
    get_serial_list()
        .into_iter()
        .map(|info| Device::from(info))
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::stream::DeviceStream;

    static DEFAULT_PRINTER: &str = "Silicon Labs";

    fn default_printer(devices: Vec<Device>) -> Option<Device> {
        for device in devices {
            if device.vendor() == DEFAULT_PRINTER {
                return Some(device);
            }
        }
        None
    }

    #[test]
    fn test_system_port() {
        let devices = list_devices();
        println!("{:#?}", &devices);

        let printer: Device = default_printer(devices).unwrap();

        let buff_capacity = std::mem::size_of::<u8>() * 100_000;
        let stream = DeviceStream::new(printer.port().unwrap(), buff_capacity);

        stream.listen().unwrap();
    }
}
