use super::Frame;
use byteorder::BigEndian;
use byteorder::ByteOrder;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde::Serialize;
use std::any::Any;
use std::convert::TryFrom;

#[derive(Debug, Serialize, Clone)]
enum FrameData {
    TempsAndCycles {
        id: u32,
        timestamp: DateTime<Utc>,
        data: TempsAndCycles,
    },
    HealthAndMinMaxTemps {
        id: u32,
        timestamp: DateTime<Utc>,
        data: HealthAndMinMaxTemps,
    },
    VoltageAndCurrent {
        id: u32,
        timestamp: DateTime<Utc>,
        data: VoltageAndCurrent,
    },
    StatusFlags {
        id: u32,
        timestamp: DateTime<Utc>,
        data: StatusFlags,
    },
    CellMinMax {
        id: u32,
        timestamp: DateTime<Utc>,
        data: CellMinMax,
    },
    CellTemps {
        id: u32,
        timestamp: DateTime<Utc>,
        data: CellTemps,
    },
    CellVoltages0 {
        id: u32,
        timestamp: DateTime<Utc>,
        data: CellVoltages,
    },
    CellVoltages1 {
        id: u32,
        timestamp: DateTime<Utc>,
        data: CellVoltages,
    },
    CellVoltages2 {
        id: u32,
        timestamp: DateTime<Utc>,
        data: CellVoltages,
    },
    CellVoltages3 {
        id: u32,
        timestamp: DateTime<Utc>,
        data: CellVoltages,
    },
    ACCChargeKiloWattHours {
        id: u32,
        timestamp: DateTime<Utc>,
        data: ACCChargeKiloWattHours,
    },
}

trait FrameMetadata {
    fn get_id(&self) -> u32;
}

impl FrameMetadata for FrameData {
    fn get_id(&self) -> u32 {
        match self {
            FrameData::TempsAndCycles {
                id,
                timestamp: _,
                data: _,
            } => *id,
            FrameData::HealthAndMinMaxTemps {
                id,
                timestamp: _,
                data: _,
            } => *id,
            FrameData::VoltageAndCurrent {
                id,
                timestamp: _,
                data: _,
            } => *id,
            FrameData::StatusFlags {
                id,
                timestamp: _,
                data: _,
            } => *id,
            FrameData::CellMinMax {
                id,
                timestamp: _,
                data: _,
            } => *id,
            FrameData::CellTemps {
                id,
                timestamp: _,
                data: _,
            } => *id,
            FrameData::CellVoltages0 {
                id,
                timestamp: _,
                data: _,
            } => *id,
            FrameData::CellVoltages1 {
                id,
                timestamp: _,
                data: _,
            } => *id,
            FrameData::CellVoltages2 {
                id,
                timestamp: _,
                data: _,
            } => *id,
            FrameData::CellVoltages3 {
                id,
                timestamp: _,
                data: _,
            } => *id,
            FrameData::ACCChargeKiloWattHours {
                id,
                timestamp: _,
                data: _,
            } => *id,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
struct TempsAndCycles {
    pub mos_temp: f32,
    pub board_temp: f32,
    pub cg_cg_cycle: u16,
}

impl TryFrom<&Frame> for TempsAndCycles {
    type Error = String;

    fn try_from(frame: &Frame) -> Result<Self, String> {
        if frame.data.len() < 8 {
            return Err("Invalid data frame length for TempsAndCycles".to_owned());
        }

        Ok(TempsAndCycles {
            mos_temp: (BigEndian::read_u16(&frame.data[..2]) - 400) as f32 * 0.1f32,
            board_temp: (BigEndian::read_u16(&frame.data[2..5]) - 400) as f32 * 0.1f32,
            cg_cg_cycle: BigEndian::read_u16(&frame.data[5..7]),
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
struct HealthAndMinMaxTemps {
    pub max_temp: f32,
    pub max_temp_no: u16,
    pub min_temp: f32,
    pub min_temp_no: u16,
    pub soc: u16,
    pub soh: u16,
}

impl TryFrom<&Frame> for HealthAndMinMaxTemps {
    type Error = String;

    fn try_from(frame: &Frame) -> Result<Self, Self::Error> {
        if frame.data.len() < 8 {
            return Err("Invalid data frame length for HealthAndMinMaxTemps".to_owned());
        }

        Ok(HealthAndMinMaxTemps {
            max_temp: (BigEndian::read_u16(&frame.data[..2]) - 400) as f32 * 0.1f32,
            max_temp_no: BigEndian::read_u16(&frame.data[2..3]),
            min_temp: (BigEndian::read_u16(&frame.data[3..5]) - 400) as f32 * 0.1f32,
            min_temp_no: BigEndian::read_u16(&frame.data[5..6]),
            soh: BigEndian::read_u16(&frame.data[6..7]),
            soc: BigEndian::read_u16(&frame.data[7..8]),
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
struct VoltageAndCurrent {
    pub voltage: f32,
    pub current: f32,
    pub average_cell_value: u16,
    pub max_discharge_current: f32,
}

impl TryFrom<&Frame> for VoltageAndCurrent {
    type Error = String;

    fn try_from(frame: &Frame) -> Result<Self, Self::Error> {
        if frame.data.len() < 8 {
            return Err("Invalid data frame length for VoltageAndCurrent".to_owned());
        }

        Ok(VoltageAndCurrent {
            voltage: BigEndian::read_u16(&frame.data[..2]) as f32 * 0.01f32,
            current: (BigEndian::read_u16(&frame.data[2..4]) - 4000) as f32 * 0.1f32,
            average_cell_value: BigEndian::read_u16(&frame.data[4..6]),
            max_discharge_current: (BigEndian::read_u16(&frame.data[6..8]) - 400) as f32 * 0.1f32,
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
struct StatusFlags {
    pub cell_voltage_cut_high: bool,
    pub cell_voltage_cut_low: bool,
    pub voltage_cut_high: bool,
    pub voltage_cut_low: bool,
    pub temp_cut_high: bool,
    pub temp_cut_low: bool,
    pub current_cut_high: bool,
    pub cell_voltage_alarm_high: bool,
    pub cell_voltage_alarm_low: bool,
    pub voltage_alarm_high: bool,
    pub voltage_alarm_low: bool,
    pub temp_alarm_high: bool,
    pub temp_alarm_low: bool,
    pub current_alarm_high: bool,
    pub discharge_mos_state: bool,
    pub charge_mos_state: bool,
    pub current_short: bool,
    pub power_down_state: bool,
    pub mos_temp_high: bool,
    pub internal_communication_error: bool,
    pub fan_failure_protection: bool,
    pub balancing: bool,
    pub current_cut_high_discharge: bool,
    pub current_cut_high_charge: bool,
    pub current_alarm_high_discharge: bool,
    pub current_alarm_high_charge: bool,
    pub cell_voltage_difference_cut_high: bool,
    pub cell_voltage_difference_alarm_high: bool,
    pub temp_difference_cut_high: bool,
    pub temp_difference_alarm_high: bool,
    pub invalid_cell: bool,
    pub invalid_temp: bool,
    pub current_controller_state: bool,
    pub charge_mos_error: bool,
    pub discharge_mos_error: bool,
}

impl TryFrom<&Frame> for StatusFlags {
    type Error = String;

    fn try_from(frame: &Frame) -> Result<Self, Self::Error> {
        if frame.data.len() < 8 {
            return Err("Invalid data frame length for StatusFlags".to_owned());
        }

        Ok(StatusFlags {
            cell_voltage_cut_high: (&frame.data[0] & 1) != 0,
            cell_voltage_cut_low: (&frame.data[0] & 2) >> 1 != 0,
            voltage_cut_high: (&frame.data[0] & 4) >> 2 != 0,
            voltage_cut_low: (&frame.data[0] & 8) >> 3 != 0,
            temp_cut_high: (&frame.data[0] & 0x10) >> 4 != 0,
            temp_cut_low: (&frame.data[0] & 0x20) >> 5 != 0,
            current_cut_high: (&frame.data[0] & 0x40) >> 6 != 0,
            cell_voltage_alarm_high: (&frame.data[1] & 1) != 0,
            cell_voltage_alarm_low: (&frame.data[1] & 2) >> 1 != 0,
            voltage_alarm_high: (&frame.data[1] & 4) >> 2 != 0,
            voltage_alarm_low: (&frame.data[1] & 8) >> 3 != 0,
            temp_alarm_high: (&frame.data[1] & 0x10) >> 4 != 0,
            temp_alarm_low: (&frame.data[1] & 0x20) >> 5 != 0,
            current_alarm_high: (&frame.data[1] & 0x40) >> 6 != 0,
            discharge_mos_state: (&frame.data[2] & 1) != 0,
            charge_mos_state: (&frame.data[2] & 2) >> 1 != 0,
            current_short: (&frame.data[2] & 4) >> 2 != 0,
            power_down_state: (&frame.data[2] & 8) >> 3 != 0,
            mos_temp_high: (&frame.data[2] & 0x10) >> 4 != 0,
            internal_communication_error: (&frame.data[2] & 0x20) >> 5 != 0,
            fan_failure_protection: (&frame.data[2] & 0x40) >> 6 != 0,
            balancing: (&frame.data[2] & 0x80) >> 7 != 0,
            current_cut_high_discharge: (&frame.data[3] & 1) != 0,
            current_cut_high_charge: (&frame.data[3] & 2) >> 1 != 0,
            current_alarm_high_discharge: (&frame.data[3] & 4) >> 2 != 0,
            current_alarm_high_charge: (&frame.data[3] & 8) >> 3 != 0,
            cell_voltage_difference_cut_high: (&frame.data[3] & 0x10) >> 4 != 0,
            cell_voltage_difference_alarm_high: (&frame.data[3] & 0x20) >> 5 != 0,
            temp_difference_cut_high: (&frame.data[3] & 0x40) >> 6 != 0,
            temp_difference_alarm_high: (&frame.data[3] & 0x80) >> 7 != 0,
            invalid_cell: (&frame.data[4] & 1) != 0,
            invalid_temp: (&frame.data[4] & 2) >> 1 != 0,
            current_controller_state: (&frame.data[4] & 4) >> 2 != 0,
            charge_mos_error: (&frame.data[4] & 0x10) >> 4 != 0,
            discharge_mos_error: (&frame.data[4] & 0x20) >> 5 != 0,
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
struct CellMinMax {
    pub max_cell_value: u16,
    pub max_cell_no: u8,
    pub min_cell_value: u16,
    pub min_cell_no: u8,
    pub max_charge_current: f32,
}

impl TryFrom<&Frame> for CellMinMax {
    type Error = String;

    fn try_from(frame: &Frame) -> Result<Self, Self::Error> {
        if frame.data.len() < 8 {
            return Err("Invalid data frame length for CellMinMax".to_owned());
        }

        Ok(CellMinMax {
            max_cell_value: BigEndian::read_u16(&frame.data[..2]),
            max_cell_no: frame.data[2],
            min_cell_value: BigEndian::read_u16(&frame.data[3..5]),
            min_cell_no: frame.data[5],
            max_charge_current: BigEndian::read_u16(&frame.data[..2]) as f32 / 10.0f32,
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
struct CellVoltages {
    voltages: [u16; 4],
}

impl TryFrom<&Frame> for CellVoltages {
    type Error = String;

    fn try_from(frame: &Frame) -> Result<Self, Self::Error> {
        if frame.data.len() < 8 {
            return Err("Invalid data frame length for CellVoltages".to_owned());
        }

        let mut voltages: [u16; 4] = [0; 4];
        voltages[0] = BigEndian::read_u16(&frame.data[..2]);
        voltages[1] = BigEndian::read_u16(&frame.data[2..4]);
        voltages[2] = BigEndian::read_u16(&frame.data[4..6]);
        voltages[3] = BigEndian::read_u16(&frame.data[6..8]);

        Ok(CellVoltages { voltages })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
struct CellTemps {
    temperatures: [f32; 4],
}

impl TryFrom<&Frame> for CellTemps {
    type Error = String;

    fn try_from(frame: &Frame) -> Result<Self, Self::Error> {
        if frame.data.len() < 8 {
            return Err("Invalid data frame length for CellVoltages".to_owned());
        }

        let mut temperatures: [f32; 4] = [0f32; 4];
        temperatures[0] = (BigEndian::read_u16(&frame.data[..2]) - 400) as f32 * 0.1f32;
        temperatures[1] = (BigEndian::read_u16(&frame.data[2..4]) - 400) as f32 * 0.1f32;
        temperatures[2] = (BigEndian::read_u16(&frame.data[4..6]) - 400) as f32 * 0.1f32;
        temperatures[3] = (BigEndian::read_u16(&frame.data[6..8]) - 400) as f32 * 0.1f32;

        Ok(CellTemps { temperatures })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
struct ACCChargeKiloWattHours {
    value: f32,
}

impl TryFrom<&Frame> for ACCChargeKiloWattHours {
    type Error = String;

    fn try_from(frame: &Frame) -> Result<Self, Self::Error> {
        if frame.data.len() < 4 {
            return Err("Invalid data frame length for ACCChargeKiloWattHours".to_owned());
        }

        let value: f32 = BigEndian::read_f32(&frame.data[0..5]) / 100.0f32;

        Ok(ACCChargeKiloWattHours { value })
    }
}

macro_rules! frame_result {
    ($frame:expr, $module_id:expr, $variant:ident, $type:ident) => {
        match $type::try_from($frame) {
            Ok(data) => Ok(FrameData::$variant {
                id: $module_id,
                timestamp: Utc::now(),
                data: data,
            }),
            Err(error) => Err(error),
        }
    };
}

fn try_to_decode_frame(frame: &Frame) -> Result<FrameData, String> {
    match frame.id {
        418517265 => frame_result!(frame, 0, CellVoltages0, CellVoltages),
        418517266 => frame_result!(frame, 0, CellVoltages1, CellVoltages),
        418517267 => frame_result!(frame, 0, CellVoltages2, CellVoltages),
        418517268 => frame_result!(frame, 0, CellVoltages3, CellVoltages),
        418517269 => frame_result!(frame, 0, CellTemps, CellTemps),
        418517270 => frame_result!(frame, 0, TempsAndCycles, TempsAndCycles),
        418517271 => frame_result!(frame, 0, ACCChargeKiloWattHours, ACCChargeKiloWattHours),
        418517281 => frame_result!(frame, 0, VoltageAndCurrent, VoltageAndCurrent),
        418517282 => frame_result!(frame, 0, CellMinMax, CellMinMax),
        418517283 => frame_result!(frame, 0, HealthAndMinMaxTemps, HealthAndMinMaxTemps),
        418517284 => frame_result!(frame, 0, StatusFlags, StatusFlags),
        418517521 => frame_result!(frame, 1, CellVoltages0, CellVoltages),
        418517522 => frame_result!(frame, 1, CellVoltages1, CellVoltages),
        418517523 => frame_result!(frame, 1, CellVoltages2, CellVoltages),
        418517524 => frame_result!(frame, 1, CellVoltages3, CellVoltages),
        418517525 => frame_result!(frame, 1, CellTemps, CellTemps),
        418517526 => frame_result!(frame, 1, TempsAndCycles, TempsAndCycles),
        418517527 => frame_result!(frame, 1, ACCChargeKiloWattHours, ACCChargeKiloWattHours),
        418517537 => frame_result!(frame, 1, VoltageAndCurrent, VoltageAndCurrent),
        418517538 => frame_result!(frame, 1, CellMinMax, CellMinMax),
        418517539 => frame_result!(frame, 1, HealthAndMinMaxTemps, HealthAndMinMaxTemps),
        418517540 => frame_result!(frame, 1, StatusFlags, StatusFlags),
        418517777 => frame_result!(frame, 2, CellVoltages0, CellVoltages),
        418517778 => frame_result!(frame, 2, CellVoltages1, CellVoltages),
        418517779 => frame_result!(frame, 2, CellVoltages2, CellVoltages),
        418517780 => frame_result!(frame, 2, CellVoltages3, CellVoltages),
        418517781 => frame_result!(frame, 2, CellTemps, CellTemps),
        418517782 => frame_result!(frame, 2, TempsAndCycles, TempsAndCycles),
        418517783 => frame_result!(frame, 2, ACCChargeKiloWattHours, ACCChargeKiloWattHours),
        418517793 => frame_result!(frame, 2, VoltageAndCurrent, VoltageAndCurrent),
        418517794 => frame_result!(frame, 2, CellMinMax, CellMinMax),
        418517795 => frame_result!(frame, 2, HealthAndMinMaxTemps, HealthAndMinMaxTemps),
        418517796 => frame_result!(frame, 2, StatusFlags, StatusFlags),
        418518033 => frame_result!(frame, 3, CellVoltages0, CellVoltages),
        418518034 => frame_result!(frame, 3, CellVoltages1, CellVoltages),
        418518035 => frame_result!(frame, 3, CellVoltages2, CellVoltages),
        418518036 => frame_result!(frame, 3, CellVoltages3, CellVoltages),
        418518037 => frame_result!(frame, 3, CellTemps, CellTemps),
        418518038 => frame_result!(frame, 3, TempsAndCycles, TempsAndCycles),
        418518039 => frame_result!(frame, 3, ACCChargeKiloWattHours, ACCChargeKiloWattHours),
        418518049 => frame_result!(frame, 3, VoltageAndCurrent, VoltageAndCurrent),
        418518050 => frame_result!(frame, 3, CellMinMax, CellMinMax),
        418518051 => frame_result!(frame, 3, HealthAndMinMaxTemps, HealthAndMinMaxTemps),
        418518052 => frame_result!(frame, 3, StatusFlags, StatusFlags),
        _ => Err("Unknown ID frame".to_owned()),
    }
}

#[derive(Debug, Default)]
pub struct DynessCanProtocol {
    data: Vec<FrameData>,
}

impl DynessCanProtocol {
    fn find_index_of_frame_data(&self, target: &FrameData) -> Option<usize> {
        for (index, item) in self.data.iter().enumerate() {
            if item.get_id() == target.get_id() && item.type_id() == target.type_id() {
                return Some(index);
            }
        }
        None
    }

    pub fn decode(&mut self, frame: &Frame) {
        if let Ok(frame_data) = try_to_decode_frame(frame) {
            if let Some(index) = self.find_index_of_frame_data(&frame_data) {
                // A frame with the same ID exists, update it!
                self.data[index] = frame_data;
            } else {
                // This frame do not exists, add it!
                self.data.push(frame_data);
            }
        }
    }
}
