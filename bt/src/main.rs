mod inverter;
use bluer::Address;
use dotenv::dotenv;
use inverter::bt::{BTInterface, InfluxData};

#[tokio::main]
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

    let bt_interface = BTInterface::new(Address::new(device_address), influx_data);
    let _ = bt_interface
        .serve(bt_period, bt_night_period)
        .await
        .map_err(|e| {
            println!("Error: {} {}", e.kind.to_string(), e.message);
        });

    println!("Bluetooth power watch closed.");
}
