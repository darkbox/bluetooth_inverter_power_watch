use super::Frame;
use byteorder::{BigEndian, ByteOrder};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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
            board_temp: (BigEndian::read_u16(&frame.data[2..4]) - 400) as f32 * 0.1f32,
            cg_cg_cycle: BigEndian::read_u16(&frame.data[4..6]),
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
            max_temp_no: frame.data[2] as u16,
            min_temp: (BigEndian::read_u16(&frame.data[3..5]) - 400) as f32 * 0.1f32,
            min_temp_no: frame.data[5] as u16,
            soh: frame.data[6] as u16,
            soc: frame.data[7] as u16,
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
            max_discharge_current: (BigEndian::read_u16(&frame.data[6..8]) as i32 - 400) as f32
                * 0.1f32,
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

enum MessageType {
    CellVoltages0,
    CellVoltages1,
    CellVoltages2,
    CellVoltages3,
    CellTemps,
    TempsAndCycles,
    ACCChargeKiloWattHours,
    VoltageAndCurrent,
    CellMinMax,
    HealthAndMinMaxTemps,
    StatusFlags,
}

impl TryFrom<u8> for MessageType {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x11 => Ok(MessageType::CellVoltages0),
            0x12 => Ok(MessageType::CellVoltages1),
            0x13 => Ok(MessageType::CellVoltages2),
            0x14 => Ok(MessageType::CellVoltages3),
            0x15 => Ok(MessageType::CellTemps),
            0x16 => Ok(MessageType::TempsAndCycles),
            0x17 => Ok(MessageType::ACCChargeKiloWattHours),
            0x21 => Ok(MessageType::VoltageAndCurrent),
            0x22 => Ok(MessageType::CellMinMax),
            0x23 => Ok(MessageType::HealthAndMinMaxTemps),
            0x24 => Ok(MessageType::StatusFlags),
            _ => Err(format!("Unknown message type: {}", value)),
        }
    }
}

fn try_to_decode_frame(frame: &Frame) -> Result<FrameData, String> {
    // Looks like protocol is working as: base_id + module * 0x100 + message_type

    const BASE_ADDR: u8 = 0x0A;
    let addr = ((frame.id >> 8) & 0xFF) as u8;
    if addr < BASE_ADDR {
        return Err("Invalid module address".into());
    }

    let module = (addr - BASE_ADDR) as u32;
    let msg_byte = (frame.id & 0xFF) as u8;
    let msg = MessageType::try_from(msg_byte)?;

    match msg {
        MessageType::CellVoltages0 => frame_result!(frame, module, CellVoltages0, CellVoltages),
        MessageType::CellVoltages1 => frame_result!(frame, module, CellVoltages1, CellVoltages),
        MessageType::CellVoltages2 => frame_result!(frame, module, CellVoltages2, CellVoltages),
        MessageType::CellVoltages3 => frame_result!(frame, module, CellVoltages3, CellVoltages),
        MessageType::CellTemps => frame_result!(frame, module, CellTemps, CellTemps),
        MessageType::TempsAndCycles => frame_result!(frame, module, TempsAndCycles, TempsAndCycles),
        MessageType::ACCChargeKiloWattHours => frame_result!(
            frame,
            module,
            ACCChargeKiloWattHours,
            ACCChargeKiloWattHours
        ),
        MessageType::VoltageAndCurrent => {
            frame_result!(frame, module, VoltageAndCurrent, VoltageAndCurrent)
        }
        MessageType::CellMinMax => frame_result!(frame, module, CellMinMax, CellMinMax),
        MessageType::HealthAndMinMaxTemps => {
            frame_result!(frame, module, HealthAndMinMaxTemps, HealthAndMinMaxTemps)
        }
        MessageType::StatusFlags => frame_result!(frame, module, StatusFlags, StatusFlags),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ModuleId(pub u8);

impl Into<ModuleId> for u32 {
    fn into(self) -> ModuleId {
        ModuleId(self as u8)
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct DynessBatteryModule {
    voltage_and_current: Option<VoltageAndCurrent>,
    health: Option<HealthAndMinMaxTemps>,
    status: Option<StatusFlags>,
    temps_and_cycles: Option<TempsAndCycles>,
    cell_min_max: Option<CellMinMax>,
    cell_temps: Option<CellTemps>,
    cell_voltages: [Option<CellVoltages>; 4],
    acc_charge_kwh: Option<ACCChargeKiloWattHours>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DynessCanProtocol {
    modules: HashMap<ModuleId, DynessBatteryModule>,
    last_update: Option<DateTime<Utc>>,
}

impl DynessCanProtocol {
    /// Decode a CAN frame and update the internal state of the protocol.
    pub fn decode(&mut self, frame: &Frame) {
        let Ok(frame_data) = try_to_decode_frame(frame) else {
            // Skip unknown frame types
            return;
        };

        let module = self.modules.entry(frame_data.get_id().into()).or_default();
        self.last_update = Some(Utc::now());

        match frame_data {
            FrameData::VoltageAndCurrent { data, .. } => {
                module.voltage_and_current = Some(data);
            }

            FrameData::HealthAndMinMaxTemps { data, .. } => {
                module.health = Some(data);
            }

            FrameData::StatusFlags { data, .. } => {
                module.status = Some(data);
            }

            FrameData::TempsAndCycles { data, .. } => {
                module.temps_and_cycles = Some(data);
            }

            FrameData::CellMinMax { data, .. } => {
                module.cell_min_max = Some(data);
            }

            FrameData::CellTemps { data, .. } => {
                module.cell_temps = Some(data);
            }

            FrameData::CellVoltages0 { data, .. } => {
                module.cell_voltages[0] = Some(data);
            }

            FrameData::CellVoltages1 { data, .. } => {
                module.cell_voltages[1] = Some(data);
            }

            FrameData::CellVoltages2 { data, .. } => {
                module.cell_voltages[2] = Some(data);
            }

            FrameData::CellVoltages3 { data, .. } => {
                module.cell_voltages[3] = Some(data);
            }

            FrameData::ACCChargeKiloWattHours { data, .. } => {
                module.acc_charge_kwh = Some(data);
            }
        }
    }

    /// Average SOC across all modules.
    pub fn average_soc(&self) -> Option<f32> {
        let mut total = 0.0;
        let mut count = 0;

        for module in self.modules.values() {
            if let Some(health) = &module.health {
                total += health.soc as f32;
                count += 1;
            }
        }

        if count == 0 {
            None
        } else {
            Some(total / count as f32)
        }
    }

    /// Number of battery modules currently seen.
    pub fn module_count(&self) -> usize {
        self.modules.len()
    }

    /// Iterator over all discovered modules.
    #[allow(dead_code)]
    pub fn modules(&self) -> impl Iterator<Item = (&ModuleId, &DynessBatteryModule)> {
        self.modules.iter()
    }

    /// Serialize the current state of the protocol to JSON.
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        // Include the average SOC and module count in the JSON output
        let mut json_map = serde_json::Map::new();
        json_map.insert(
            "average_soc".to_string(),
            serde_json::json!(self.average_soc()),
        );
        json_map.insert(
            "module_count".to_string(),
            serde_json::json!(self.module_count()),
        );
        json_map.insert("modules".to_string(), serde_json::json!(self.modules));
        serde_json::to_string(&json_map)
    }
}
