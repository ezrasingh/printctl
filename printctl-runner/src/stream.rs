use std::time::Duration;

use gcode::{GCode, Mnemonic};
use serial::core::{SerialDevice, SerialPort};

pub struct DeviceStream<T: SerialDevice + std::marker::Sync> {
    device: T,
    buff: Vec<u8>,
}

impl<T: SerialDevice + std::marker::Sync> DeviceStream<T> {
    pub fn new(device: T, buffer_capacity: usize) -> Self {
        DeviceStream {
            device,
            buff: Vec::with_capacity(buffer_capacity),
        }
    }

    pub fn listen(mut self) -> std::io::Result<()> {
        self.device.reconfigure(&|settings| {
            settings.set_baud_rate(serial::Baud9600)?;
            settings.set_char_size(serial::Bits8);
            settings.set_parity(serial::ParityNone);
            settings.set_stop_bits(serial::Stop1);
            settings.set_flow_control(serial::FlowNone);
            Ok(())
        })?;

        self.device.set_timeout(Duration::from_millis(1000))?;

        let src = r#"
            G90              (absolute coordinates)
            G00 X50.0 Y-10   (move somewhere)
        "#;

        for _gcode in gcode::parse(src) {
            self.device.write_all(_gcode.to_string().as_bytes())?;
        }

        self.device.read(&mut self.buff[..])?;
        Ok(())
    }
}
