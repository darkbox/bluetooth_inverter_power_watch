mod inverter;
mod usb_can_battery;
mod config {
    pub const INFLUXDB2_HOST: &str = "INFLUXDB2_HOST";
    pub const INFLUXDB2_ORG: &str = "INFLUXDB2_ORG";
    pub const INFLUXDB2_BUCKET: &str = "INFLUXDB2_BUCKET";
    pub const INFLUXDB2_API_TOKEN: &str = "INFLUXDB2_API_TOKEN";

    pub const POOLING_PERIOD: &str = "POOLING_PERIOD";
    pub const POOLING_NIGHT_PERIOD: &str = "POOLING_NIGHT_PERIOD";

    pub const INVERTER_BT_ADDRESS: &str = "INVERTER_BT_ADDRESS";

    pub const WEB_SERVER_PORT: &str = "WEB_SERVER_PORT";

    pub const CANBUS_DEBUG_MSGS: &str = "CANBUS_DEBUG_MSGS";
    pub const CANBUS_TTY_DEVICE: &str = "CANBUS_TTY_DEVICE";
    pub const CANBUS_TTY_BAUD_RATE: &str = "CANBUS_TTY_BAUD_RATE";

    pub const DEFAULT_WEB_SERVER_PORT: u16 = 9999;
    pub const DEFAULT_CANBUS_BAUD_RATE: u32 = 2_000_000;
    pub const DEFAULT_BT_PERIOD: u64 = 30;
    pub const DEFAULT_BT_NIGHT_PERIOD: u64 = 300;
}

use crate::usb_can_battery::{Decoder, DynessBatteryStatus, FrameType};
use actix_cors::Cors;
use actix_web::{get, rt, web, App, HttpResponse, HttpServer, Responder};
use bluer::Address;
use dotenvy::dotenv;
use inverter::{
    bt::{BTInterface, InfluxData},
    InverterData,
};
use std::{io, sync::Arc, sync::RwLock, thread, time::Duration};
use tokio::{runtime::Handle, signal};

#[derive(Clone)]
pub struct AppState {
    inverter: Arc<RwLock<Option<InverterData>>>,
    battery: Arc<RwLock<Option<DynessBatteryStatus>>>,
}

#[actix_web::main]
async fn main() {
    println!("Starting bluetooth power watch...");

    dotenv().expect(".env file not found.");
    let influxdb2_host =
        std::env::var(config::INFLUXDB2_HOST).expect("INFLUXDB2_HOST must be set.");
    let influxdb2_org = std::env::var(config::INFLUXDB2_ORG).expect("INFLUXDB2_ORG must be set.");
    let influxdb2_bucket =
        std::env::var(config::INFLUXDB2_BUCKET).expect("INFLUXDB2_BUCKET must be set.");
    let influxdb2_api_token =
        std::env::var(config::INFLUXDB2_API_TOKEN).expect("INFLUXDB2_API_TOKEN must be set.");
    let bt_period = std::env::var(config::POOLING_PERIOD).expect("POOLING_PERIOD must be set.");
    let bt_period = bt_period
        .parse::<u64>()
        .unwrap_or(config::DEFAULT_BT_PERIOD);
    let bt_night_period =
        std::env::var(config::POOLING_NIGHT_PERIOD).expect("POOLING_NIGHT_PERIOD must be set.");
    let bt_night_period = bt_night_period
        .parse::<u64>()
        .unwrap_or(config::DEFAULT_BT_NIGHT_PERIOD);
    let device_address =
        std::env::var(config::INVERTER_BT_ADDRESS).expect("INVERTER_BT_ADDRESS must be set.");
    let device_address: Vec<u8> = device_address
        .split(':')
        .map(|x| hex::decode(x).expect("Invalid bluetooth address")[0])
        .collect();
    let device_address: [u8; 6] = device_address
        .try_into()
        .expect("Invalid bluetooth address length");
    let influx_data = InfluxData::new(
        influxdb2_host,
        influxdb2_org,
        influxdb2_api_token,
        influxdb2_bucket,
    );

    let web_server_port = std::env::var(config::WEB_SERVER_PORT)
        .ok()
        .and_then(|v| v.parse::<u16>().ok())
        .unwrap_or(config::DEFAULT_WEB_SERVER_PORT);

    let state = AppState {
        inverter: Arc::new(RwLock::new(None)),
        battery: Arc::new(RwLock::new(None)),
    };

    // Run Web Service
    println!("Starting web server...");
    let web_server_host = "0.0.0.0".to_owned();
    let web_state = state.clone();
    let server = HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .app_data(web::Data::new(web_state.clone()))
            .service(root)
            .service(json_response_inverter_info)
            .service(json_response_can_battery_info)
    })
    .bind((web_server_host.clone(), web_server_port))
    .unwrap()
    .run();
    rt::spawn(server);
    println!(
        "Server running at http://{}:{}",
        web_server_host, web_server_port
    );

    let can_bus_influx_data = influx_data.clone();

    // Run Bluetooth Service
    println!("Starting bluetooth interface service...");
    let mut bt_interface = BTInterface::new(Address::new(device_address), influx_data);
    let state_for_bt = state.clone();
    bt_interface.connect(move |data| on_emit(&state_for_bt, data));
    rt::spawn(async move {
        let _ = bt_interface.serve(bt_period, bt_night_period).await;
    });

    // Run USB CAN serial port Service
    let handle = Handle::current();
    let can_debug = std::env::var(config::CANBUS_DEBUG_MSGS)
        .ok()
        .and_then(|v| v.parse::<bool>().ok())
        .unwrap_or(false);

    if let Ok(port_name) = std::env::var(config::CANBUS_TTY_DEVICE) {
        let baud_rate: u32 = std::env::var(config::CANBUS_TTY_BAUD_RATE)
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(config::DEFAULT_CANBUS_BAUD_RATE);

        println!("Starting USB CAN serial interface service...");
        thread::spawn(move || {
            let expire_time = std::time::Duration::from_secs(5 * 60); // 5min

            let mut dyness_protocol = usb_can_battery::dyness::DynessCanProtocol::default();

            loop {
                let expire_connection = std::time::SystemTime::now();

                let mut port = serialport::new(&port_name, baud_rate)
                    .timeout(Duration::from_millis(10))
                    .open()
                    .expect("Failed to open port");

                let mut serial_buf: Vec<u8> = vec![0; 1];
                let mut decoder = Decoder::new();
                println!("Receiving data on {} at {} baud:", &port_name, &baud_rate);

                loop {
                    // Read serial port data
                    match port.read_exact(serial_buf.as_mut_slice()) {
                        Ok(_) => {
                            if let Some(frame) = decoder.append(serial_buf[0]) {
                                match frame.id {
                                    787 => {
                                        if can_debug {
                                            println!("|====>{}", frame.to_string());
                                        }
                                        let battery_status = DynessBatteryStatus::from(frame);
                                        if let Ok(battery_status) = battery_status {
                                            if can_debug {
                                                println!("{}", battery_status.to_string());
                                            }

                                            let battery_status_copy: DynessBatteryStatus =
                                                battery_status.clone();

                                            *state.battery.write().unwrap() = Some(battery_status);

                                            if can_debug {
                                                println!("Saving CAN bus data into DB...");
                                            }
                                            let influx_data_copy = can_bus_influx_data.clone();
                                            handle.spawn(async move {
                                                battery_status_copy
                                                    .save_to_db(influx_data_copy)
                                                    .await;
                                            });
                                        }
                                    }
                                    _ => {
                                        if can_debug
                                            && frame.header.frame_type == FrameType::Standard
                                        {
                                            println!("{}", frame.to_string());
                                        }

                                        // TODO: Untested
                                        dyness_protocol.decode(&frame);
                                        println!("{:?}", &dyness_protocol);
                                    }
                                }
                            }
                        }
                        Err(ref e) if e.kind() == io::ErrorKind::TimedOut => {
                            if let Ok(secs) = expire_connection.elapsed() {
                                if secs > expire_time {
                                    // Connection has expired. Restart it!
                                    break;
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("{:?}", e);
                            // An error has occurred. Restart it!
                            break;
                        }
                    }
                }

                drop(port);
                thread::sleep(Duration::from_secs(3));
            }
        });
    }

    // Close on Ctrl+c
    match signal::ctrl_c().await {
        Ok(()) => {
            println!("Bluetooth power watch closed.");
        }
        Err(err) => {
            eprintln!("Unable to listen for shutdown signal: {}", err);
            // we also shut down in case of error
        }
    }
}

/// Callback function to handle emitted inverter data and update the application state.
fn on_emit(state: &AppState, data: InverterData) {
    *state.inverter.write().unwrap() = Some(data);
}

#[get("/")]
async fn root() -> impl Responder {
    let html_bytes = include_bytes!("web/index.html");
    HttpResponse::Ok()
        .content_type("text/html")
        .body(String::from_utf8_lossy(html_bytes))
}

#[get("/api/info")]
async fn json_response_inverter_info(state: web::Data<AppState>) -> impl Responder {
    let guard = state.battery.read().unwrap();

    match guard.as_ref().and_then(|data| data.to_json().ok()) {
        Some(json) => HttpResponse::Ok()
            .content_type("application/json")
            .body(json),
        None => HttpResponse::Ok().body("NO DATA"),
    }
}

#[get("/api/info/battery")]
async fn json_response_can_battery_info(state: web::Data<AppState>) -> impl Responder {
    let guard = state.battery.read().unwrap();

    match guard.as_ref().and_then(|data| data.to_json().ok()) {
        Some(json) => HttpResponse::Ok()
            .content_type("application/json")
            .body(json),
        None => HttpResponse::Ok().body("NO DATA"),
    }
}
