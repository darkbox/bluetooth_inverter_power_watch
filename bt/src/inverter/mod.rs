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
const CHAR_UUID_0X2A0C: uuid::Uuid = uuid::Uuid::from_u128(0x00002a0c00001000800000805f9b34fb);
const CHAR_UUID_0X2A0D: uuid::Uuid = uuid::Uuid::from_u128(0x00002a0d00001000800000805f9b34fb);
const CHAR_UUID_0X2A11: uuid::Uuid = uuid::Uuid::from_u128(0x00002a1100001000800000805f9b34fb);
const CHAR_UUID_0X2A12: uuid::Uuid = uuid::Uuid::from_u128(0x00002a1200001000800000805f9b34fb);
const CHAR_UUID_0X2A13: uuid::Uuid = uuid::Uuid::from_u128(0x00002a1300001000800000805f9b34fb);
const CHAR_UUID_0X2A14: uuid::Uuid = uuid::Uuid::from_u128(0x00002a1400001000800000805f9b34fb);

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct InverterData {
    // Product info
    model_type: u8,
    topology: String,
    cpu_version: String,
    blt_version: String,

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
    watts_unkown_01: u16,
    unkown_02: f32,
    unkown_03: u16,

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

    fn b_to_f32(bytes: [u8; 2]) -> f32 {
        u16::from_le_bytes(bytes) as f32 * 0.1
    }

    fn parse_0x2a01(&mut self, bytes: Vec<u8>) {
        self.cpu_version = String::from_utf8_lossy(&bytes[3..11]).into_owned();
        self.blt_version = String::from_utf8_lossy(&bytes[12..]).into_owned();
    }

    fn parse_0x2a02(&mut self, _bytes: Vec<u8>) {
        // NOTE: Not needed for us.
        //       This is used to download a logreport from the inverter.
    }

    fn parse_0x2a03(&mut self, bytes: Vec<u8>) {
        self.ac_voltage = InverterData::b_to_f32([bytes[0], bytes[1]]);
        self.ac_frequency = InverterData::b_to_f32([bytes[2], bytes[3]]);
        self.output_voltage = InverterData::b_to_f32([bytes[4], bytes[5]]);
        self.output_frequency = InverterData::b_to_f32([bytes[6], bytes[7]]);
        self.output_apparent_power = u16::from_le_bytes([bytes[8], bytes[9]]);
        self.output_active_power = u16::from_le_bytes([bytes[10], bytes[11]]);
        self.load_percentage = u16::from_le_bytes([bytes[12], bytes[13]]);
        self.unkown_02 = InverterData::b_to_f32([bytes[14], bytes[15]]);
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

        // Boolean parameters
        // TODO decode missing parameters
        // let params: [u8; 4] = [
        //     bytes[11].to_be_bytes()[0],
        //     bytes[10].to_be_bytes()[0],
        //     bytes[9].to_be_bytes()[0],
        //     bytes[8].to_be_bytes()[0],
        // ];
        // println!("Params: {:?}", params);
    }

    fn parse_0x2a05(&mut self, bytes: Vec<u8>) {
        self.model_type = bytes[16];
        self.nominal_ac_voltage = InverterData::b_to_f32([bytes[0], bytes[1]]);
        self.nominal_ac_current = InverterData::b_to_f32([bytes[8], bytes[9]]);
        self.rated_battery_voltage = InverterData::b_to_f32([bytes[14], bytes[15]]);
        self.nominal_output_voltage = InverterData::b_to_f32([bytes[4], bytes[5]]);
        self.nominal_output_frequency = InverterData::b_to_f32([bytes[6], bytes[7]]);
        self.nominal_output_apparent_power = u16::from_le_bytes([bytes[10], bytes[11]]);
        self.nominal_output_active_power = u16::from_le_bytes([bytes[12], bytes[13]]);
    }

    fn parse_0x2a0c(&mut self, bytes: Vec<u8>) {
        self.p_output_voltage = u16::from_le_bytes([bytes[0], bytes[1]]);
        self.p_output_frequency = InverterData::b_to_f32([bytes[2], bytes[3]]);
        self.p_max_charging_current = bytes[4] & 0xff;
        self.p_max_ac_charging_current = bytes[5] & 0xff;
        self.p_back_to_grid_voltage = InverterData::b_to_f32([bytes[12], bytes[13]]);

        // if p_back_to_discharge == 0.0 => "FULL"
        self.p_back_to_discharge_voltage = InverterData::b_to_f32([bytes[14], bytes[15]]);
        self.p_bulk_charging_voltage = InverterData::b_to_f32([bytes[8], bytes[9]]);
        self.p_float_charging_voltage = InverterData::b_to_f32([bytes[6], bytes[7]]);
        self.p_battery_cutoff_voltage = InverterData::b_to_f32([bytes[10], bytes[11]]);
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
        self.p_equalization_voltage = InverterData::b_to_f32([bytes[10], bytes[11]]);
        // TODO self.p_rt_activate_battery_equalization = ; // Real-time activate battery equalization
    }

    fn parse_0x2a11(&mut self, bytes: Vec<u8>) {
        self.pv_input_voltage_stage1 = InverterData::b_to_f32([bytes[12], bytes[13]]);
        self.pv_input_power_stage1 = u16::from_le_bytes([bytes[14], bytes[15]]);
    }

    fn parse_0x2a12(&mut self, bytes: Vec<u8>) {
        self.pv_input_voltage_stage2 = InverterData::b_to_f32([bytes[12], bytes[13]]);
        self.pv_input_power_stage2 = u16::from_le_bytes([bytes[14], bytes[15]]);
    }

    fn parse_0x2a13(&mut self, bytes: Vec<u8>) {
        self.pv_input_voltage_stage3 = InverterData::b_to_f32([bytes[12], bytes[13]]);
        self.pv_input_power_stage3 = u16::from_le_bytes([bytes[14], bytes[15]]);
    }

    fn parse_0x2a14(&mut self, bytes: Vec<u8>) {
        self.pv_input_voltage_stage4 = InverterData::b_to_f32([bytes[12], bytes[13]]);
        self.pv_input_power_stage4 = u16::from_le_bytes([bytes[14], bytes[15]]);
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
