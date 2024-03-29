mod inverter;
mod usb_can_battery;

use crate::usb_can_battery::{Decoder, DynessBatteryStatus, FrameType};
use actix_cors::Cors;
use actix_web::{get, rt, App, HttpResponse, HttpServer, Responder};
use bluer::Address;
use dotenvy::dotenv;
use inverter::{
    bt::{BTInterface, InfluxData},
    InverterData,
};
use std::{io, thread, time::Duration};
use tokio::{signal, runtime::Handle};

// FIXME: Use mutex or something secure
static mut SHARED_BUFFER: Option<InverterData> = None;
static mut SHARED_BATTERY_STATUS: Option<DynessBatteryStatus> = None;

//#[tokio::main]
#[actix_web::main]
async fn main() {
    println!("Starting bluetooth power watch...");

    dotenv().expect(".env file not found.");
    let influxdb2_host = std::env::var("INFLUXDB2_HOST").expect("INFLUXDB2_HOST must be set.");
    let influxdb2_org = std::env::var("INFLUXDB2_ORG").expect("INFLUXDB2_ORG must be set.");
    let influxdb2_bucket =
        std::env::var("INFLUXDB2_BUCKET").expect("INFLUXDB2_BUCKET must be set.");
    let influxdb2_api_token =
        std::env::var("INFLUXDB2_API_TOKEN").expect("INFLUXDB2_API_TOKEN must be set.");
    let bt_period = std::env::var("POOLING_PERIOD").expect("POOLING_PERIOD must be set.");
    let bt_period = bt_period.parse::<u64>().unwrap_or(30);
    let bt_night_period =
        std::env::var("POOLING_NIGHT_PERIOD").expect("POOLING_NIGHT_PERIOD must be set.");
    let bt_night_period = bt_night_period.parse::<u64>().unwrap_or(300);
    let device_address =
        std::env::var("INVERTER_BT_ADDRESS").expect("INVERTER_BT_ADDRESS must be set.");
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
    let mut web_server_port: u16 = 9999;
    if let Ok(value) = std::env::var("WEB_SERVER_PORT") {
        web_server_port = value.parse::<u16>().unwrap_or(web_server_port);
    }

    // Run Web Service
    println!("Starting web server...");
    let server = HttpServer::new(|| {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .service(root)
            .service(json_response_inverter_info)
            .service(json_response_can_battery_info)
    })
    .bind(("0.0.0.0", web_server_port))
    .unwrap()
    .run();
    rt::spawn(server);
    println!("Server running at http://localhost:{}", web_server_port);

    let can_bus_influx_data = influx_data.clone();

    // Run Bluetooth Service
    println!("Starting bluetooth interface service...");
    let mut bt_interface = BTInterface::new(Address::new(device_address), influx_data);
    bt_interface.connect(on_emit);
    rt::spawn(async move {
        let _ = bt_interface.serve(bt_period, bt_night_period).await;
    });

    // Run USB CAN serial port Service
    let handle = Handle::current();

    let can_debug = std::env::var("CANBUS_DEBUG_MSGS").unwrap_or("false".to_owned());
    let can_debug: bool = if can_debug == "true" { true } else { false };
    let port_name = std::env::var("CANBUS_TTY_DEVICE");
    if port_name.is_ok() {
        let port_name = port_name.unwrap();
        let mut baud_rate = 2_000_000;
        if let Ok(value) = std::env::var("CANBUS_TTY_BAUD_RATE") {
            baud_rate = value.parse::<u32>().unwrap_or(baud_rate);
        }
        println!("Starting USB CAN serial interface service...");
        thread::spawn(move || {
            let expire_time = std::time::Duration::from_secs(5 * 60); // 5min          

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

                                            let battery_status_copy: DynessBatteryStatus = battery_status.clone();
                                            
                                            unsafe {
                                                SHARED_BATTERY_STATUS = Some(battery_status);
                                            }
                                            
                                            if can_debug {
                                                println!("Saving CAN bus data into DB...");
                                            }
                                            let influx_data_copy = can_bus_influx_data.clone();
                                            handle.spawn(async move {
                                                battery_status_copy.save_to_db(influx_data_copy).await;
                                            });
                                        }
                                    }
                                    _ => {
                                        if can_debug && frame.header.frame_type == FrameType::Standard {
                                            println!("{}", frame.to_string());
                                        }
                                    }
                                }
                            }
                        }
                        Err(ref e) if e.kind() == io::ErrorKind::TimedOut => {
                            if let Ok(secs) = expire_connection.elapsed() {
                               if secs > expire_time {
                                    // Conection has expired. Restart it!
                                    break;
                               }
                            }
                        },
                        Err(e) => {
                             eprintln!("{:?}", e);
                             // An error has ocurred. Restart it!
                             break;
                        },
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

fn on_emit(data: InverterData) {
    unsafe {
        SHARED_BUFFER = Some(data);
    }
}

#[get("/")]
async fn root() -> impl Responder {
    let html_bytes = include_bytes!("web/index.html");
    return HttpResponse::Ok()
        .content_type("text/html")
        .body(String::from_utf8_lossy(html_bytes));
}

#[get("/api/info")]
async fn json_response_inverter_info() -> impl Responder {
    unsafe {
        if let Some(data) = &SHARED_BUFFER {
            if let Ok(json) = data.to_json() {
                return HttpResponse::Ok()
                    .content_type("application/json")
                    .body(json);
            }
        }
    }
    return HttpResponse::Ok().body("NO DATA");
}

#[get("/api/info/battery")]
async fn json_response_can_battery_info() -> impl Responder {
    unsafe {
        if let Some(data) = &SHARED_BATTERY_STATUS {
            if let Ok(json) = data.to_json() {
                return HttpResponse::Ok()
                    .content_type("application/json")
                    .body(json);
            }
        }
    }

    return HttpResponse::Ok().body("NO DATA");
}
