use crate::inverter::InverterData;
use bluer::agent::{AgentHandle, ReqResult, RequestPasskey};
use bluer::{agent::Agent, Adapter, AdapterEvent, Address};
use chrono::{Local, NaiveTime};
use futures::prelude::*;
use futures::{pin_mut, StreamExt};
use influxdb2::models::DataPoint;
use influxdb2::Client;
use std::cell::RefCell;
use std::time::Duration;
use tokio::time::sleep;

#[derive(Clone)]
pub struct InfluxData {
    host: String,
    org: String,
    token: String,
    bucket: String,
}

impl InfluxData {
    pub fn new(host: String, org: String, token: String, bucket: String) -> Self {
        InfluxData {
            host,
            org,
            token,
            bucket,
        }
    }
}

#[derive(Clone)]
pub struct BTInterface {
    target_device: Address,
    data: RefCell<InverterData>,
    influx_data: InfluxData,
    listener: Option<fn(data: InverterData)>,
}

unsafe impl Sync for BTInterface {}

impl BTInterface {
    #[allow(dead_code)]
    pub fn new(target_device: Address, influx_data: InfluxData) -> Self {
        BTInterface {
            target_device,
            data: RefCell::new(InverterData::new()),
            influx_data,
            listener: None,
        }
    }

    pub async fn serve(&self, period: u64, night_period: u64) {
        loop {
            let _ = self.scan_and_query_once().await.map_err(|e| {
                println!("Error: {} {}", e.kind.to_string(), e.message);
            });
            self.save_to_db().await;

            if let Some(data) = self.get_data() {
                self.emit(data);
            }

            let low = NaiveTime::from_hms_opt(7, 15, 0).unwrap();
            let high = NaiveTime::from_hms_opt(23, 15, 0).unwrap();
            let time_of_day = Local::now().time();
            let mut current_period: u64 = night_period;
            if (time_of_day > low) && (time_of_day < high) {
                current_period = period;
            }

            if current_period > 0 {
                sleep(Duration::from_secs(current_period)).await;
            }
        }
    }

    pub fn connect(&mut self, listener: fn(data: InverterData)) {
        self.listener = Some(listener);
    }

    fn emit(&self, data: InverterData) {
        if let Some(listener) = self.listener {
            listener(data);
        }
    }

    async fn save_to_db(&self) {
        let points = self.get_data_points();
        let client = Client::new(
            &self.influx_data.host,
            &self.influx_data.org,
            &self.influx_data.token,
        );
        let result = client
            .write(&self.influx_data.bucket, stream::iter(points))
            .await;
        if let Err(e) = result {
            println!("Influxdb client error: {}", e.to_string());
        }
    }

    fn get_data_points(&self) -> Vec<DataPoint> {
        let point_battery = DataPoint::builder("battery")
            .tag("host", "inverter")
            .field(
                "capacity",
                self.data.borrow().clone().battery_capacity as f64,
            )
            .field("voltage", self.data.borrow().clone().battery_voltage as f64)
            .field(
                "charge",
                self.data.borrow().clone().battery_charge_current as f64,
            )
            .field(
                "discharge",
                self.data.borrow().clone().battery_current_discharge as f64,
            )
            .build()
            .unwrap();

        let point_inverter = DataPoint::builder("inverter")
            .tag("host", "inverter")
            .field(
                "pv1_power",
                self.data.borrow().clone().pv_input_power_stage1 as f64,
            )
            .field(
                "pv1_voltage",
                self.data.borrow().clone().pv_input_voltage_stage1 as f64,
            )
            .field(
                "output_voltage",
                self.data.borrow().clone().output_voltage as f64,
            )
            .field(
                "output_power",
                self.data.borrow().clone().output_active_power as f64,
            )
            .field("load", self.data.borrow().clone().load_percentage as f64)
            .build()
            .unwrap();

        vec![point_battery, point_inverter]
    }

    #[allow(dead_code)]
    pub fn get_data(&self) -> Option<InverterData> {
        match self.data.try_borrow() {
            Ok(data) => Some(data.clone()),
            Err(_) => None,
        }
    }

    pub async fn request_passkey(_req: RequestPasskey) -> ReqResult<u32> {
        println!("Requesting passkey...");
        let device_pin_code =
            std::env::var("INVERTER_BT_PASSKEY").expect("INVERTER_BT_PASSKEY must be set.");
        let device_pin_code: u32 = device_pin_code.parse::<u32>().unwrap_or(123456);
        Ok(device_pin_code)
    }

    async fn scan_and_query_once(&self) -> bluer::Result<()> {
        let session = bluer::Session::new().await?;

        // Register custom agent to handle the authentication
        let agent = Agent {
            request_default: true,
            request_passkey: Some(Box::new(|req| Box::pin(BTInterface::request_passkey(req)))),
            ..Default::default()
        };

        #[allow(unused_variables)]
        let agent_handle: AgentHandle;
        if let Ok(handle) = session.register_agent(agent).await {
            agent_handle = handle;
            println!("Custom agent registered: {:?}", agent_handle);
        } else {
            println!("Error: Custom agent could not be registered.");
        }

        let adapter = session.default_adapter().await?;
        println!(
            "Discovering devices using Bluetooth adapter {}",
            adapter.name()
        );
        adapter.set_powered(true).await?;

        let device_events = adapter.discover_devices().await?;
        pin_mut!(device_events);

        loop {
            tokio::select! {
                Some(device_event) = device_events.next() => {
                    match device_event {
                        AdapterEvent::DeviceAdded(addr) => {
                            if addr == self.target_device {
                                // println!("Inverter device found: {}", addr);
                                let res = self.query_device(&adapter, addr).await;
                                if let Err(err) = res {
                                    println!("    Error: {}", &err);
                                }
                                break;
                            }
                        }
                        AdapterEvent::DeviceRemoved(addr) => {
                            println!("Device removed: {}", addr);
                        }
                        _ => (),
                    }
                    println!();
                }
                else => break
            }
        }

        Ok(())
    }

    async fn query_device(&self, adapter: &Adapter, addr: Address) -> bluer::Result<()> {
        let device = adapter.device(addr)?;
        device.set_trusted(true).await?;

        // println!("    Address type:       {}", device.address_type().await?);
        // println!("    Name:               {:?}", device.name().await?);
        // println!("    Paired:             {:?}", device.is_paired().await?);
        // println!("    Connected:          {:?}", device.is_connected().await?);
        // println!("    Trusted:            {:?}", device.is_trusted().await?);

        match device.is_paired().await {
            Ok(is_paired) => {
                if !is_paired {
                    // println!("Trying to pair device...");
                    device.pair().await?;
                    device.connect().await?;
                } else {
                    // println!("Connecting...");
                    device.connect().await?;
                }
            }
            Err(err) => {
                println!("    Error: {}", &err);
            }
        };

        for service in device.services().await? {
            self.data.borrow_mut().read_characteristics(service).await?;
        }

        // self.data.print_inverter_info();
        // self.data.print_battery_info();
        // self.data.print_basic_info();
        // self.data.print_parameters();
        // println!("{:?}", self.data);
        // self.data.borrow().print_json().unwrap_or_default();

        Ok(())
    }
}
