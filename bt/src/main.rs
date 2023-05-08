mod inverter;
use actix_web::{get, rt, App, HttpResponse, HttpServer, Responder};
use bluer::Address;
use dotenv::dotenv;
use inverter::{
    bt::{BTInterface, InfluxData},
    InverterData,
};
use tokio::signal;

static mut SHARED_BUFFER: Option<InverterData> = None;

//#[tokio::main]
#[actix_web::main]
async fn main() {
    println!("Starting bluetooth power watch...");

    dotenv().ok();
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
        .map(|x| hex::decode(x).expect("Invalid blutooth address")[0])
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
    let server = HttpServer::new(|| App::new().service(root).service(json_response))
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
async fn json_response() -> impl Responder {
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
