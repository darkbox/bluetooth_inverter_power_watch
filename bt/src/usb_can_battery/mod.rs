use std::slice::Iter;

use byteorder::{BigEndian, ByteOrder, LittleEndian};
use futures::stream;
use serde::{Deserialize, Serialize};
use influxdb2::models::DataPoint;

use crate::inverter::bt::InfluxData;

#[derive(Debug)]
pub struct Frame {
    pub header: FrameHeader,
    pub id: u16,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Copy)]
pub enum FrameFormat {
    DataFrame,
    RemoteFrame,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum FrameType {
    Standard,
    Extended,
}

#[derive(Debug)]
pub struct FrameHeader {
    pub frame_type: FrameType,
    frame_format: FrameFormat,
    frame_data_length: usize,
}

const PACKET_HEADER: u8 = 0xAA;
const PACKET_END: u8 = 0x55;

pub struct Decoder {
    packet_buffer: Vec<u8>,
}

impl Decoder {
    pub fn new() -> Self {
        Decoder {
            packet_buffer: Vec::new(),
        }
    }

    pub fn append(&mut self, byte: u8) -> Option<Frame> {
        if self.packet_buffer.first() != Some(&PACKET_HEADER) && byte == PACKET_HEADER {
            // Start packet
            self.packet_buffer.push(byte);
            return None;
        }

        if self.packet_buffer.first() == Some(&PACKET_HEADER) && byte != PACKET_END {
            // Packet payload
            self.packet_buffer.push(byte);
            return None;
        }

        if self.packet_buffer.first() == Some(&PACKET_HEADER) && byte == PACKET_END {
            // End packet
            self.packet_buffer.push(byte);
            // println!("{:02X?}", &self.packet_buffer);
            let frame = self.decode_frame();
            self.packet_buffer.clear();
            return frame;
        }

        None
    }

    fn decode_id(frame_type: &FrameType, iter: &mut Iter<'_, u8>) -> Result<u16, String> {
        match frame_type {
            FrameType::Standard => {
                if iter.len() < 2 {
                    return Err("Standard frame has less than 2 bytes".to_owned());
                }
                let bytes = [*iter.next().unwrap(), *iter.next().unwrap()];
                Ok(LittleEndian::read_u16(&bytes))
            }
            FrameType::Extended => {
                if iter.len() < 4 {
                    return Err("Extended frame has less than 4 bytes".to_owned());
                }
                let bytes = [
                    *iter.next().unwrap(),
                    *iter.next().unwrap(),
                    *iter.next().unwrap(),
                    *iter.next().unwrap(),
                ];
                Ok(LittleEndian::read_u16(&bytes))
            }
        }
    }

    fn decode_frame(&mut self) -> Option<Frame> {
        let mut raw = self.packet_buffer.iter();

        while raw.next() != Some(&PACKET_HEADER) {}

        let mut frame = Frame::new();
        frame.header = FrameHeader::from_bytes(&raw.next().unwrap());
        
        match Decoder::decode_id(&frame.header.frame_type, &mut raw) {
            Ok(frame_id) => {
                frame.id = frame_id;
            },
            Err(_) => {
                return None;
            }
        }

        let mut i = 0;
        while let Some(b) = raw.next() {
            i = i + 1;

            if *b == PACKET_END && i > frame.header.frame_data_length {
                return Some(frame);
            }

            frame.data.push(*b);
        }

        None
    }
}

impl Frame {
    fn new() -> Self {
        Frame {
            header: FrameHeader::default(),
            id: 0,
            data: Vec::new(),
        }
    }
}

impl Clone for Frame {
    fn clone(&self) -> Self {
        Frame {
            header: self.header.clone(),
            id: self.id.clone(),
            data: self.data.clone(),
        }
    }
}

impl ToString for Frame {
    fn to_string(&self) -> String {
        format!(
            "ID: {:?} Data: {:02X?} Len: {:?} - {:?}",
            self.id,
            self.data,
            self.data.len(),
            self.header,
        )
    }
}

impl FrameHeader {
    fn from_bytes(byte: &u8) -> Self {
        let mut frame_type = FrameType::Standard;
        if (byte & (1 << 5)) != 0 {
            frame_type = FrameType::Extended;
        }

        let mut frame_format = FrameFormat::DataFrame;
        if (byte & (1 << 4)) != 0 {
            frame_format = FrameFormat::RemoteFrame;
        }

        FrameHeader {
            frame_type,
            frame_format,
            frame_data_length: ((byte & 0b_11100000) >> 5).into(),
        }
    }
}

impl Clone for FrameHeader {
    fn clone(&self) -> Self {
        FrameHeader {
            frame_format: self.frame_format.clone(),
            frame_type: self.frame_type.clone(),
            frame_data_length: self.frame_data_length.clone(),
        }
    }
}

impl Default for FrameHeader {
    fn default() -> Self {
        FrameHeader {
            frame_type: FrameType::Standard,
            frame_format: FrameFormat::DataFrame,
            frame_data_length: 0,
        }
    }
}

impl ToString for FrameHeader {
    fn to_string(&self) -> String {
        format!(
            "Type: {:?} Format: {:?} Length: {:?}",
            self.frame_type, self.frame_format, self.frame_data_length
        )
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct DynessBatteryStatus {
    soc: u32,
    soh: u32,
    amps: f32,
    temp: f32,
    voltage: f32,
}

impl DynessBatteryStatus {
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    pub fn from(value: Frame) -> Result<Self, String> {
        // Example package data [13, B1, 00, 61, 00, A0, 50, 64]
        // 64  48      9f 00   37 01       d2 13
        // SOH SOC     TEMP    Amp?        Voltage

        if value.data.len() < 8 {
            return Err("Invalid data frame length".to_owned());
        }

        let voltage = BigEndian::read_u16(&value.data[..2]);
        let voltage: f32 = voltage as f32 * 0.01f32;
        let amps = BigEndian::read_i16(&value.data[2..4]) as f32 * 0.1f32;
        let temp = BigEndian::read_i16(&value.data[4..6]) as f32 * 0.1f32;

        Ok(DynessBatteryStatus {
            soc: LittleEndian::read_u16(&[value.data[6], 0]).into(),
            soh: LittleEndian::read_u16(&[value.data[7], 0]).into(),
            amps,
            temp,
            voltage,
        })
    }

    pub async fn save_to_db(&self, influx_data: InfluxData) {
        let points = self.get_data_points();
        let client = influx_data.create_client();
        let result = client
            .write(influx_data.get_bucket(), stream::iter(points)).await;
        
        if let Err(e) = result {
            println!("USB CAN Bus Influxdb client error: {}", e.to_string());
        }
    }

    fn get_data_points(&self) -> Vec<DataPoint> {
        let point_battery = DataPoint::builder("battery_can_bus")
            .tag("host", "battery")
            .field(
                "soc",
                self.soc as f64,
            )
            .field("voltage", self.voltage as f64)
            .field(
                "temp",
                self.temp as f64,
            )
            .field(
                "amps",
                self.amps as f64,
            )
            .build()
            .unwrap();

        vec![point_battery]
    }
}

impl ToString for DynessBatteryStatus {
    fn to_string(&self) -> String {
        format!(
            "soc: {}% soh: {}%, {}A {}V {}ºC",
            &self.soc, &self.soh, &self.amps, &self.voltage, &self.temp
        )
    }
}
