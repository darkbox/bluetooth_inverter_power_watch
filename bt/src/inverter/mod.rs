pub mod bt;

use bit_array::BitArray;
use bluer::gatt::remote::Service;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use typenum::U8;

const CHAR_UUID_0X2A01: uuid::Uuid = uuid::Uuid::from_u128(0x00002a0100001000800000805f9b34fb);
const CHAR_UUID_0X2A02: uuid::Uuid = uuid::Uuid::from_u128(0x00002a0200001000800000805f9b34fb);
const CHAR_UUID_0X2A03: uuid::Uuid = uuid::Uuid::from_u128(0x00002a0300001000800000805f9b34fb);
const CHAR_UUID_0X2A04: uuid::Uuid = uuid::Uuid::from_u128(0x00002a0400001000800000805f9b34fb);
const CHAR_UUID_0X2A05: uuid::Uuid = uuid::Uuid::from_u128(0x00002a0500001000800000805f9b34fb);
const CHAR_UUID_0X2A06: uuid::Uuid = uuid::Uuid::from_u128(0x00002a0600001000800000805f9b34fb);
const CHAR_UUID_0X2A07: uuid::Uuid = uuid::Uuid::from_u128(0x00002a0700001000800000805f9b34fb);
const CHAR_UUID_0X2A08: uuid::Uuid = uuid::Uuid::from_u128(0x00002a0800001000800000805f9b34fb);
const CHAR_UUID_0X2A09: uuid::Uuid = uuid::Uuid::from_u128(0x00002a0900001000800000805f9b34fb);
const CHAR_UUID_0X2A0B: uuid::Uuid = uuid::Uuid::from_u128(0x00002a0b00001000800000805f9b34fb);
const CHAR_UUID_0X2A0C: uuid::Uuid = uuid::Uuid::from_u128(0x00002a0c00001000800000805f9b34fb); // Parameter characteristics
const CHAR_UUID_0X2A0D: uuid::Uuid = uuid::Uuid::from_u128(0x00002a0d00001000800000805f9b34fb); // Parameter characteristics
const CHAR_UUID_0X2A0E: uuid::Uuid = uuid::Uuid::from_u128(0x00002a0e00001000800000805f9b34fb); // Parameter characteristics
const CHAR_UUID_0X2A11: uuid::Uuid = uuid::Uuid::from_u128(0x00002a1100001000800000805f9b34fb);
const CHAR_UUID_0X2A12: uuid::Uuid = uuid::Uuid::from_u128(0x00002a1200001000800000805f9b34fb);
const CHAR_UUID_0X2A13: uuid::Uuid = uuid::Uuid::from_u128(0x00002a1300001000800000805f9b34fb);
const CHAR_UUID_0X2A14: uuid::Uuid = uuid::Uuid::from_u128(0x00002a1400001000800000805f9b34fb);

// TODO send/set parameters
// const SERVICE_UUID_0X1810: uuid::Uuid = uuid::Uuid::from_u128(0x0000181000001000800000805f9b34fb);

const EVENT_MESSAGE: [&str; 32] = [
    "PV loss",
    "Inverter fault",
    "Bus Over",
    "Bus Under",
    "Bus Soft Fail",
    "Line Fail",
    "Output Short",
    "Inverter voltage too low",
    "Inverter voltage too high",
    "Over temperature",
    "Fan locked",
    "Battery voltage high",
    "Battery low alarm",
    "Over charge",
    "Battery under shutdown",
    "Battery derating",
    "Over load",
    "EEPROM Fault",
    "Inverter Over Current",
    "Inverter Soft Fail",
    "Self Test Fail",
    "OP DC Voltage Over",
    "Bat Open",
    "Current Sensor Fail",
    "Battery Short",
    "Power limit",
    "PV voltage high",
    "MPPT overload fault",
    "MPPT overload warning",
    "Battery too low to charge",
    "Reserved",
    "Reserved",
];

const EVENT_ID_01: [&str; 32] = [
    "1000", "1001", "1002", "1003", "1004", "2001", "2002", "1005", "1006", "1007", "1008", "1009",
    "2006", "2007", "2008", "2009", "1010", "2011", "1011", "1012", "1013", "1014", "1015", "1016",
    "1017", "2012", "2013", "2014", "2015", "2016", "", "",
];

const EVENT_ID_02: [&str; 32] = [
    "1000", "1001", "1002", "1003", "1004", "2001", "2002", "1005", "1006", "2003", "2004", "2005",
    "2006", "2007", "2008", "2009", "2010", "2011", "1011", "1012", "1013", "1014", "1015", "1016",
    "1017", "2012", "2013", "2014", "2015", "2016", "", "",
];

const EVENT_LEVEL: [&str; 32] = [
    "Warning", "Fault", "Fault", "Fault", "Fault", "Warning", "Fault", "Fault", "Fault", "T", "T",
    "T", "Warning", "Warning", "Warning", "Warning", "T", "Warning", "Fault", "Fault", "Fault",
    "Fault", "Warning", "Fault", "Fault", "Warning", "Warning", "Warning", "Warning", "Warning",
    "", "",
];

struct EventLine {
    id: String,
    level: String,
    message: String,
}

impl ToString for EventLine {
    fn to_string(&self) -> String {
        format!(
            "ID:{};LEVEL:{};MESSAGE:{};",
            self.id, self.level, self.message
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct InverterData {
    // Product info
    model_type: u8,
    model_identification: u8,
    topology: u8, // Note: 0 => "transformerless" 1 => "transformer". Unkown from where to get it!
    cpu_version: String,
    blt_version: String,
    operation_logic: String,

    // Basic info
    ac_voltage: f32,
    ac_frequency: f32,
    pv_input_voltage_stage1: f32,
    pv_input_power_stage1: u16,
    pv_input_voltage_stage2: f32,
    pv_input_power_stage2: u16,
    pv_input_voltage_stage3: f32,
    pv_input_power_stage3: u16,
    pv_input_voltage_stage4: f32,
    pv_input_power_stage4: u16,
    output_voltage: f32,
    output_frequency: f32,
    output_apparent_power: u16,
    output_active_power: u16,
    load_percentage: u16,

    // Modes
    output_mode: u8,
    charge_mode: u8,
    bulk_charge: i16,

    // Unknown values
    scc_cpu1: String,
    scc_cpu2: String,
    scc_cpu3: String,
    scc_cpu4: String,
    charging_data1: [u8; 32],
    charging_data2: [u8; 32],
    ac_charging_data1: [u8; 32],
    ac_charging_data2: [u8; 32],
    watts_unkown_01: u16,
    unkown_02: f32,
    unkown_03: u16,
    discharge_current: u8,

    // Rated information
    nominal_ac_voltage: f32,
    nominal_ac_current: f32,
    rated_battery_voltage: f32,
    nominal_output_voltage: f32,
    nominal_output_frequency: f32,
    nominal_output_apparent_power: u16,
    nominal_output_active_power: u16,

    // Battery info
    workmode: String,
    battery_voltage: f32,
    battery_capacity: u16,
    battery_charge_current: u16,
    battery_current_discharge: u16,

    // Battery parameters
    p_bulk_charging_voltage: f32,
    p_float_charging_voltage: f32,
    p_battery_cutoff_voltage: f32,

    // Battery equalization setting
    p_battery_equalization_enable: bool,
    p_rt_activate_battery_equalization: bool,
    p_equalization_time: u16,
    p_equalization_period: u16,
    p_equalization_timeout: u16,
    p_equalization_voltage: f32,

    // Parameters
    p_buzzer_alarm: bool,
    p_feed_into_the_grid: bool,
    p_backlight: bool,
    p_overload_auto_restart: bool,
    p_overtemp_auto_restart: bool,
    p_beeps_while_primary_source_interrupt: bool,
    p_must_be_connected_to_pv: u8,
    p_solar_power_balance: u8,
    p_overload_bypass: bool,
    p_lcd_to_default_after_one_min: bool,
    p_fault_code_record: bool,
    p_charger_source_priority: u8,
    p_output_source_priotrity: u8,
    p_ac_input_range: u8,
    p_battery_type: u8,
    p_output_frequency: f32,
    p_output_voltage: u16,
    p_back_to_grid_voltage: f32,
    p_max_charging_current: u8,
    p_max_ac_charging_current: u8,
    p_back_to_discharge_voltage: f32,
    p_min_bulk_voltage: f32,
    p_max_bulk_voltage: f32,
    p_min_undervoltage: f32,
    p_max_undervoltage: f32,
    p_bulk_charge_time_range: u8,

    // Event Log (Faults and Warnings)
    last_event_message: String,
    raw_event_flags: [u8; 32],
}

impl InverterData {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string(self)
    }

    #[allow(dead_code)]
    pub fn print_json(&self) -> Result<()> {
        println!("{}", self.to_json()?);
        Ok(())
    }

    #[allow(dead_code)]
    pub fn print_inverter_info(&self) {
        let model_type = match self.model_type {
            0 => "Grid tie".to_owned(),
            1 => "Off grid".to_owned(),
            _ => "Hybrid".to_owned(),
        };
        println!("Model type: {}", model_type);
        println!("Main CPU version: {}", self.cpu_version);
        println!("Bluetooth version: {}", self.blt_version);
    }

    #[allow(dead_code)]
    pub fn print_basic_info(&self) {
        println!("AC voltage: {}V", self.ac_voltage);
        println!("AC frequency: {}Hz", self.ac_frequency);
        println!("PV input voltage: {}V", self.pv_input_voltage_stage1);
        println!("PV input power: {}W", self.pv_input_power_stage1);
        println!("PV input voltage: {}V", self.pv_input_voltage_stage2);
        println!("PV input power: {}W", self.pv_input_power_stage2);
        println!("PV input voltage: {}V", self.pv_input_voltage_stage3);
        println!("PV input power: {}W", self.pv_input_power_stage3);
        println!("PV input voltage: {}V", self.pv_input_voltage_stage4);
        println!("PV input power: {}W", self.pv_input_power_stage4);
        println!("Output voltage: {}V", self.output_voltage);
        println!("Output frequency: {}Hz", self.output_frequency);
        println!("Output apparent power: {}VA", self.output_apparent_power);
        println!("Output active power: {}W", self.output_active_power);
        println!("Load: {}%", self.load_percentage);
        println!("Nominal AC voltage: {}V", self.nominal_ac_voltage);
        println!("Nominal AC current: {}A", self.nominal_ac_current);
        println!("Rated battery voltage: {}V", self.rated_battery_voltage);
        println!("Nominal output voltage: {}V", self.nominal_output_voltage);
        println!(
            "Nominal output frequency: {}Hz",
            self.nominal_output_frequency
        );
        println!(
            "Nominal output apparent power: {}VA",
            self.nominal_output_apparent_power
        );
        println!(
            "Nominal output active power: {}W",
            self.nominal_output_active_power
        );
    }

    #[allow(dead_code)]
    pub fn print_parameters(&self) {
        let input_voltage_range = match self.p_ac_input_range {
            0 => "Appliance",
            _ => "UPS",
        };
        println!("Input voltage range: {}", input_voltage_range);

        let charge_source_str = match self.p_charger_source_priority {
            0 => "Utility first",
            1 => "Solar first",
            2 => "Utility and Solar",
            3 => "Only Solar Charging",
            _ => "Only Solar Charging",
        };
        println!("Charge source priority: {}", charge_source_str);

        let source_output = match self.p_output_source_priotrity {
            0 => "USB Priority",
            1 => "SUB Priority",
            2 => "SBU Priority",
            _ => "SBU Priority",
        };
        println!("Source output priority: {}", source_output);

        println!("Output voltage: {}", self.p_output_voltage);

        let battery_type = match self.p_battery_type {
            0 => "AGM",
            1 => "Flooded",
            2 => "User define",
            3 => "Pylon",
            4 => "WECO",
            5 => "Other Li battery",
            _ => "Other Li battery",
        };
        println!("Battery type: {}", battery_type);

        println!("Bulk charging voltage: {}V", self.p_bulk_charging_voltage);
        println!("Float charging voltage: {}V", self.p_float_charging_voltage);
        println!(
            "Battery cut-off voltage: {}V",
            self.p_battery_cutoff_voltage
        );

        let output_mode_str = match self.output_mode {
            0 => "Single machine output",
            1 => "Parallel output",
            2 => "Phase 1 of 3 Phase output",
            3 => "Phase 2 of 3 Phase output",
            4 => "Phase 3 of 3 Phase output",
            _ => "Phase 3 of 3 Phase output",
        };
        println!("Output mode: {}", output_mode_str);

        let charge_mode_str = match self.charge_mode {
            0 => "Auto",
            1 => "2-stage",
            2 => "3-stage",
            _ => "3-stage",
        };
        println!("Charge mode: {}", charge_mode_str);

        if self.bulk_charge == -1 {
            println!("Bulk charge: Auto");
        }
    }

    #[allow(dead_code)]
    pub fn print_battery_info(&self) {
        println!("Workmode: {}", self.workmode);
        println!("Battery voltage: {}V", self.battery_voltage);
        println!("Battery capacity: {}%", self.battery_capacity);
        println!("Battery charge current: {}A", self.battery_charge_current);
        println!(
            "Battery current discharge: {}A",
            self.battery_current_discharge
        );
    }

    fn b_to_f32(bytes: [u8; 2], factor: Option<f32>) -> f32 {
        u16::from_le_bytes(bytes) as f32 * factor.unwrap_or(0.1)
    }

    fn split_bytes(bytes: &[u8]) -> Vec<u8> {
        BitArray::<u32, U8>::from_bytes(bytes)
            .into_iter()
            .map(|b| if b { 1 } else { 0 })
            .collect()
    }

    fn parse_0x2a01(&mut self, bytes: Vec<u8>) {
        self.cpu_version = String::from_utf8_lossy(&bytes[3..11]).into_owned();
        self.blt_version = String::from_utf8_lossy(&bytes[12..]).into_owned();
    }

    fn parse_0x2a02(&mut self, bytes: Vec<u8>) {
        self.model_identification = bytes[19];
    }

    fn parse_0x2a03(&mut self, bytes: Vec<u8>) {
        self.ac_voltage = InverterData::b_to_f32([bytes[0], bytes[1]], None);
        self.ac_frequency = InverterData::b_to_f32([bytes[2], bytes[3]], None);
        self.output_voltage = InverterData::b_to_f32([bytes[4], bytes[5]], None);
        self.output_frequency = InverterData::b_to_f32([bytes[6], bytes[7]], None);
        self.output_apparent_power = u16::from_le_bytes([bytes[8], bytes[9]]);
        self.output_active_power = u16::from_le_bytes([bytes[10], bytes[11]]);
        self.load_percentage = u16::from_le_bytes([bytes[12], bytes[13]]);
        self.unkown_02 = InverterData::b_to_f32([bytes[14], bytes[15]], None);
        self.battery_voltage = u16::from_le_bytes([bytes[16], bytes[17]]) as f32 * 0.01;
        self.battery_charge_current = u16::from_le_bytes([bytes[18], bytes[19]]);
    }

    fn parse_0x2a04(&mut self, bytes: Vec<u8>) {
        let workmode_binding = [bytes[12]];
        self.workmode = String::from_utf8_lossy(&workmode_binding).into_owned();

        if bytes[13] != 1 {
            self.watts_unkown_01 = u16::from_le_bytes([bytes[14], bytes[15]]);
        }

        self.battery_capacity = u16::from_le_bytes([bytes[0], bytes[1]]);
        self.unkown_03 = u16::from_le_bytes([bytes[2], bytes[3]]);
        self.battery_current_discharge = u16::from_le_bytes([bytes[4], bytes[5]]);

        // Fault & Warning codes
        let mut event: Vec<u8> = Vec::new();
        event.append(&mut InverterData::split_bytes(&[bytes[8]]));
        event.append(&mut InverterData::split_bytes(&[bytes[9]]));
        event.append(&mut InverterData::split_bytes(&[bytes[10]]));
        event.append(&mut InverterData::split_bytes(&[bytes[11]]));
        // event.reverse();

        // println!("Event flags: {:?}", event);

        self.last_event_message = InverterData::parse_event_message(&event).unwrap_or_default();
        if event.len() == 32 {
            self.raw_event_flags = event.try_into().unwrap_or_default();
        }
    }

    fn parse_0x2a05(&mut self, bytes: Vec<u8>) {
        self.model_type = bytes[16];
        self.nominal_ac_voltage = InverterData::b_to_f32([bytes[0], bytes[1]], None);
        self.nominal_ac_current = InverterData::b_to_f32([bytes[8], bytes[9]], None);
        self.rated_battery_voltage = InverterData::b_to_f32([bytes[14], bytes[15]], None);
        self.nominal_output_voltage = InverterData::b_to_f32([bytes[4], bytes[5]], None);
        self.nominal_output_frequency = InverterData::b_to_f32([bytes[6], bytes[7]], None);
        self.nominal_output_apparent_power = u16::from_le_bytes([bytes[10], bytes[11]]);
        self.nominal_output_active_power = u16::from_le_bytes([bytes[12], bytes[13]]);
    }

    fn parse_0x2a06(&mut self, bytes: Vec<u8>) {
        // TODO What contains this package? Is it a string?
        // self.charging_data1 = String::from_utf8_lossy(&bytes).to_string();
        if bytes.len() == 32 {
            self.charging_data1 = bytes.try_into().unwrap_or_default();
        }
    }

    fn parse_0x2a07(&mut self, bytes: Vec<u8>) {
        // TODO What contains this package? Is it a string?
        // self.charging_data2 = String::from_utf8_lossy(&bytes).to_string();
        if bytes.len() == 32 {
            self.charging_data2 = bytes.try_into().unwrap_or_default();
        }
    }

    fn parse_0x2a08(&mut self, bytes: Vec<u8>) {
        // TODO What contains this package? Is it a string?
        //self.ac_charging_data1 = String::from_utf8_lossy(&bytes).to_string();
        if bytes.len() == 32 {
            self.ac_charging_data1 = bytes.try_into().unwrap_or_default();
        }
    }

    fn parse_0x2a09(&mut self, bytes: Vec<u8>) {
        // TODO What contains this package? Is it a string?
        // self.ac_charging_data2 = String::from_utf8_lossy(&bytes).to_string();
        if bytes.len() == 32 {
            self.ac_charging_data2 = bytes.try_into().unwrap_or_default();
        }
    }

    fn parse_0x2a0b(&mut self, bytes: Vec<u8>) {
        self.p_min_bulk_voltage = InverterData::b_to_f32([bytes[6], bytes[7]], None);
        self.p_max_bulk_voltage = InverterData::b_to_f32([bytes[8], bytes[9]], None);
        self.p_min_undervoltage = InverterData::b_to_f32([bytes[10], bytes[11]], None);
        self.p_max_undervoltage = InverterData::b_to_f32([bytes[12], bytes[13]], None);
        self.p_bulk_charge_time_range = bytes[5]; // if 0 => "AUTO" else "Number value"

        // Flags
        // let flags_01 = BitArray::<u32, U8>::from_bytes(&[bytes[14]]);
        // TODO parse this flags
    }

    fn parse_0x2a0c(&mut self, bytes: Vec<u8>) {
        self.p_output_voltage = u16::from_le_bytes([bytes[0], bytes[1]]);
        self.p_output_frequency = InverterData::b_to_f32([bytes[2], bytes[3]], None);
        self.p_max_charging_current = bytes[4] & 0xff;
        self.p_max_ac_charging_current = bytes[5] & 0xff;
        self.p_back_to_grid_voltage = InverterData::b_to_f32([bytes[12], bytes[13]], None);

        // if p_back_to_discharge == 0.0 => "FULL"
        self.p_back_to_discharge_voltage = InverterData::b_to_f32([bytes[14], bytes[15]], None);
        self.p_bulk_charging_voltage = InverterData::b_to_f32([bytes[8], bytes[9]], None);
        self.p_float_charging_voltage = InverterData::b_to_f32([bytes[6], bytes[7]], None);
        self.p_battery_cutoff_voltage = InverterData::b_to_f32([bytes[10], bytes[11]], None);
        self.p_charger_source_priority = bytes[18];
        self.p_ac_input_range = bytes[16];
        self.p_output_source_priotrity = bytes[17];
        self.p_battery_type = bytes[19];
    }

    fn parse_0x2a0d(&mut self, bytes: Vec<u8>) {
        self.output_mode = bytes[2];
        self.charge_mode = bytes[15];
        self.bulk_charge = i16::from_le_bytes([bytes[16], bytes[17]]);

        // Flags
        let flags_01 = BitArray::<u32, U8>::from_bytes(&[bytes[0]]);
        let flags_02 = BitArray::<u32, U8>::from_bytes(&[bytes[1]]);

        let flag_01 = flags_02.get(7).unwrap_or_default();
        let flag_02 = flags_01.get(1).unwrap_or_default();
        let flag_03 = flags_01.get(5).unwrap_or_default();
        let flag_04 = flags_01.get(3).unwrap_or_default();
        let flag_05 = flags_01.get(4).unwrap_or_default();
        let flag_06 = flags_01.get(6).unwrap_or_default();
        let flag_29 = flags_02.get(6).unwrap_or_default();
        let flag_30 = flags_01.get(0).unwrap_or_default();
        let flag_31 = flags_01.get(2).unwrap_or_default();
        let flag_32 = flags_01.get(7).unwrap_or_default();
        let flag_21 = bytes[3];
        let flag_22 = bytes[4];

        /* TODO [flag_10 == 0 => No permited | flag_30 == 1 => Enabled | flag_30 == 0 => Disabled] */
        // let flag_10 = flags_02.get(5).unwrap_or_default();

        self.p_buzzer_alarm = flag_01; // Buzzer alarm
        self.p_feed_into_the_grid = flag_02; // Feed into the grid
        self.p_backlight = flag_03; // Backlight
        self.p_overload_auto_restart = flag_04; // Overload auto restart
        self.p_overtemp_auto_restart = flag_05; // Over temperature auto restart
        self.p_beeps_while_primary_source_interrupt = flag_06; // Beeps while primary source interrupt
        self.p_must_be_connected_to_pv = flag_21; // All inverters must connected to PV as PV OK
        self.p_solar_power_balance = flag_22; // Solar power balance
        self.p_battery_equalization_enable = flag_29; // Battery equalization setting
        self.p_overload_bypass = flag_30; // Overload bypass
        self.p_lcd_to_default_after_one_min = flag_31; // LCD screen returns to default display screen after 1 min.
        self.p_fault_code_record = flag_32; // Fault code record

        self.p_equalization_time = u16::from_le_bytes([bytes[6], bytes[7]]); // minutes
        self.p_equalization_period = u16::from_le_bytes([bytes[8], bytes[9]]); // days
        self.p_equalization_timeout = u16::from_le_bytes([bytes[12], bytes[13]]); // minutes
        self.p_equalization_voltage = InverterData::b_to_f32([bytes[10], bytes[11]], Some(0.01));
        // TODO self.p_rt_activate_battery_equalization = ; // Real-time activate battery equalization
    }

    fn parse_0x2a0e(&mut self, bytes: Vec<u8>) {
        self.operation_logic = match bytes[0] {
            0 => "AUTO".to_owned(),
            1 => "ONLINE".to_owned(),
            2 => "ECO".to_owned(),
            _ => format!("UNKNOWN ({:?})", bytes[0]),
        };

        // NOTE: Discharge current when max discharge current is enabled?
        self.discharge_current = bytes[1]; // Is Amps
    }

    fn parse_0x2a11(&mut self, bytes: Vec<u8>) {
        self.scc_cpu1 = String::from_utf8_lossy(&bytes[0..8]).into_owned();
        self.pv_input_voltage_stage1 = InverterData::b_to_f32([bytes[12], bytes[13]], None);
        self.pv_input_power_stage1 = u16::from_le_bytes([bytes[14], bytes[15]]);
    }

    fn parse_0x2a12(&mut self, bytes: Vec<u8>) {
        self.scc_cpu2 = String::from_utf8_lossy(&bytes[0..8]).into_owned();
        self.pv_input_voltage_stage2 = InverterData::b_to_f32([bytes[12], bytes[13]], None);
        self.pv_input_power_stage2 = u16::from_le_bytes([bytes[14], bytes[15]]);
    }

    fn parse_0x2a13(&mut self, bytes: Vec<u8>) {
        self.scc_cpu3 = String::from_utf8_lossy(&bytes[0..8]).into_owned();
        self.pv_input_voltage_stage3 = InverterData::b_to_f32([bytes[12], bytes[13]], None);
        self.pv_input_power_stage3 = u16::from_le_bytes([bytes[14], bytes[15]]);
    }

    fn parse_0x2a14(&mut self, bytes: Vec<u8>) {
        self.scc_cpu4 = String::from_utf8_lossy(&bytes[0..8]).into_owned();
        self.pv_input_voltage_stage4 = InverterData::b_to_f32([bytes[12], bytes[13]], None);
        self.pv_input_power_stage4 = u16::from_le_bytes([bytes[14], bytes[15]]);
    }

    fn parse_event_message(event: &Vec<u8>) -> Option<String> {
        for i in 0..event.len() {
            let mut event_line = EventLine {
                id: EVENT_ID_01[i].to_string(),
                level: EVENT_LEVEL[i].to_string(),
                message: EVENT_MESSAGE[i].to_string(),
            };

            if event[i] == 1 && EVENT_LEVEL[i] != "" {
                if EVENT_LEVEL[i] == "T" {
                    if event[i] == 1 {
                        event_line.id = EVENT_ID_01[i].to_string();
                        event_line.level = "Fault".to_owned();
                    } else {
                        event_line.id = EVENT_ID_02[i].to_string();
                        event_line.level = "Warning".to_owned();
                    }
                }

                return Some(event_line.to_string());
            }

            // if i != 1 {
            //     return Some(event_line.to_string());
            // }
        }

        None
    }

    pub async fn read_characteristics(&mut self, service: Service) -> bluer::Result<()> {
        for char in service.characteristics().await? {
            let uuid = char.uuid().await?;

            match uuid {
                CHAR_UUID_0X2A01 => {
                    if char.flags().await?.read {
                        let value = char.read().await?;
                        self.parse_0x2a01(value);
                    }
                }
                CHAR_UUID_0X2A02 => {
                    if char.flags().await?.read {
                        let value = char.read().await?;
                        self.parse_0x2a02(value);
                    }
                }
                CHAR_UUID_0X2A03 => {
                    if char.flags().await?.read {
                        let value = char.read().await?;
                        self.parse_0x2a03(value);
                    }
                }
                CHAR_UUID_0X2A04 => {
                    if char.flags().await?.read {
                        let value = char.read().await?;
                        self.parse_0x2a04(value);
                    }
                }
                CHAR_UUID_0X2A05 => {
                    if char.flags().await?.read {
                        let value = char.read().await?;
                        self.parse_0x2a05(value);
                    }
                }
                CHAR_UUID_0X2A06 => {
                    if char.flags().await?.read {
                        let value = char.read().await?;
                        self.parse_0x2a06(value);
                    }
                }
                CHAR_UUID_0X2A07 => {
                    if char.flags().await?.read {
                        let value = char.read().await?;
                        self.parse_0x2a07(value);
                    }
                }
                CHAR_UUID_0X2A08 => {
                    if char.flags().await?.read {
                        let value = char.read().await?;
                        self.parse_0x2a08(value);
                    }
                }
                CHAR_UUID_0X2A09 => {
                    if char.flags().await?.read {
                        let value = char.read().await?;
                        self.parse_0x2a09(value);
                    }
                }
                CHAR_UUID_0X2A0B => {
                    if char.flags().await?.read {
                        let value = char.read().await?;
                        self.parse_0x2a0b(value);
                    }
                }
                CHAR_UUID_0X2A0C => {
                    if char.flags().await?.read {
                        let value = char.read().await?;
                        self.parse_0x2a0c(value);
                    }
                }
                CHAR_UUID_0X2A0D => {
                    if char.flags().await?.read {
                        let value = char.read().await?;
                        self.parse_0x2a0d(value);
                    }
                }
                CHAR_UUID_0X2A0E => {
                    if char.flags().await?.read {
                        let value = char.read().await?;
                        self.parse_0x2a0e(value);
                    }
                }
                CHAR_UUID_0X2A11 => {
                    if char.flags().await?.read {
                        let value = char.read().await?;
                        self.parse_0x2a11(value);
                    }
                }
                CHAR_UUID_0X2A12 => {
                    if char.flags().await?.read {
                        let value = char.read().await?;
                        self.parse_0x2a12(value);
                    }
                }
                CHAR_UUID_0X2A13 => {
                    if char.flags().await?.read {
                        let value = char.read().await?;
                        self.parse_0x2a13(value);
                    }
                }
                CHAR_UUID_0X2A14 => {
                    if char.flags().await?.read {
                        let value = char.read().await?;
                        self.parse_0x2a14(value);
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }
}
