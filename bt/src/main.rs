mod inverter;
mod usb_can_battery;

use std::{io, thread, time::Duration};

use actix_cors::Cors;
use actix_web::{get, http, rt, App, HttpResponse, HttpServer, Responder};
use bluer::Address;
use dotenvy::dotenv;
use inverter::{
    bt::{BTInterface, InfluxData},
    InverterData,
};
use tokio::signal;
use usb_can_battery::{server::UsbCanInterface, DynessBatteryStatus};

use crate::usb_can_battery::{Decoder, FrameType};

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
        // let cors = Cors::default()
        //     .allowed_origin("*")
        //     .allowed_methods(vec!["GET", "POST"])
        //     .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
        //     .allowed_header(http::header::CONTENT_TYPE)
        //     .max_age(3600);

        App::new()
            // .wrap(cors)
            .service(root)
            .service(json_response_inverter_info)
            .service(json_response_can_battery_info)
    })
    .bind(("0.0.0.0", web_server_port))
    .unwrap()
    .run();
    rt::spawn(server);
    println!("Server running at http://localhost:{}", web_server_port);

    // Run Bluetooth Service
    println!("Starting bluetooth interface service...");
    let mut bt_interface = BTInterface::new(Address::new(device_address), influx_data);
    bt_interface.connect(on_emit);
    rt::spawn(async move {
        let _ = bt_interface.serve(bt_period, bt_night_period).await;
    });

    // Run USB CAN serial port Service
    // let port_name = std::env::var("CANBUS_TTY_DEVICE");
    // if port_name.is_ok() {
    //     let port_name = port_name.unwrap();
    //     let mut baud_rate: u32 = 200_000;
    //     if let Ok(value) = std::env::var("CANBUS_TTY_BAUD_RATE") {
    //         baud_rate = value.parse::<u32>().unwrap_or(baud_rate);
    //     }
    //     println!("Starting USB CAN serial interface service...");
    //     let mut uci = UsbCanInterface::new(port_name, baud_rate);
    //     uci.connect(on_received_battery_status);
    //     thread::spawn(move || {
    //         uci.serve();
    //     });
    // }
    let port_name = "/dev/ttyVUSB0";
    let baud_rate = 2_000_000;

    thread::spawn(move || {
        let mut port = serialport::new(port_name, baud_rate)
            .timeout(Duration::from_millis(10))
            .open()
            .expect("Failed to open port");

        let mut serial_buf: Vec<u8> = vec![0; 1];
        let mut decoder = Decoder::new();
        println!("Receiving data on {} at {} baud:", &port_name, &baud_rate);
        loop {
            match port.read_exact(serial_buf.as_mut_slice()) {
                Ok(_) => {
                    if let Some(frame) = decoder.append(serial_buf[0]) {
                        match frame.id {
                            787 => {
                                println!("|====>{}", frame.to_string());
                                let battery_status = DynessBatteryStatus::from(frame);
                                println!("{}", battery_status.to_string());
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
    });

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

fn on_received_battery_status(data: DynessBatteryStatus) {
    unsafe {
        SHARED_BATTERY_STATUS = Some(data);
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
