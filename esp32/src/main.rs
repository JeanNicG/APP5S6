pub mod ble_scan;
pub mod websocket;
pub mod coap_server;
pub mod wifi;

use anyhow::Result;
use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::hal::gpio::PinDriver;
use esp_idf_svc::hal::task::block_on;
use esp_idf_svc::wifi::EspWifi;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::nvs::EspDefaultNvsPartition;

use std::sync::{Arc, Mutex, mpsc};
use std::thread;



fn main() -> Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take()?;
    let sys_loop = EspSystemEventLoop::take()?;
    let nvs = EspDefaultNvsPartition::take()?;

    // Setup Wi-Fi
    let mut wifi = EspWifi::new(peripherals.modem, sys_loop.clone(), Some(nvs))?;
    wifi::connect_wifi(&mut wifi)?;

    // Setup LED for CoAP
    let led = Arc::new(Mutex::new(PinDriver::output(peripherals.pins.gpio2)?));
    
    // Start CoAP Server thread
    let coap_led = led.clone();
    thread::spawn(move || {
        let _ = coap_server::run_coap_server(coap_led);
    });

    // Start BLE scan thread
    let (tx, rx) = mpsc::sync_channel::<String>(10);
    thread::spawn(move || {
        block_on(ble_scan::run_ble_scan(tx))
    });

    // Run WebSocket Server for the relay
    websocket::run_websocket_server(rx);

    Ok(())
}