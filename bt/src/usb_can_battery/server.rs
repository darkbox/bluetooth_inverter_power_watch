use std::{io, time::Duration};

use crate::usb_can_battery::{Decoder, DynessBatteryStatus, FrameType};

pub struct UsbCanInterface {
    port_name: String,
    baud_rate: u32,
    listener: Option<fn(data: DynessBatteryStatus)>,
}

impl UsbCanInterface {
    pub fn new<'a>(port_name: impl Into<std::borrow::Cow<'a, str>>, baud_rate: u32) -> Self {
        UsbCanInterface {
            port_name: port_name.into().into_owned(),
            baud_rate,
            listener: None,
        }
    }

    pub fn serve(&self) {
        let port = serialport::new(&self.port_name, self.baud_rate)
            .timeout(Duration::from_millis(10))
            .open();

        match port {
            Ok(mut port) => {
                let mut serial_buf: Vec<u8> = vec![0; 1];
                let mut decoder = Decoder::new();
                println!(
                    "Receiving data on {} at {} baud:",
                    &self.port_name, &self.baud_rate
                );

                loop {
                    match port.read_exact(serial_buf.as_mut_slice()) {
                        Ok(_) => {
                            if let Some(frame) = decoder.append(serial_buf[0]) {
                                match frame.id {
                                    787 => {
                                        println!("|====>{}", frame.to_string());
                                        let battery_status = DynessBatteryStatus::from(frame);
                                        println!("{}", battery_status.to_string());
                                        self.emit(battery_status);
                                    }
                                    _ => {
                                        if frame.header.frame_type == FrameType::Standard {
                                            println!("{}", frame.to_string());
                                        }
                                    }
                                }
                            }
                        }
                        Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                        Err(e) => eprintln!("{:?}", e),
                    }
                }
            }
            Err(_) => {
                println!("Failed to open port: {}", &self.port_name);
            }
        }
    }

    pub fn connect(&mut self, listener: fn(data: DynessBatteryStatus)) {
        self.listener = Some(listener);
    }

    fn emit(&self, data: DynessBatteryStatus) {
        if let Some(listener) = self.listener {
            listener(data);
        }
    }
}
