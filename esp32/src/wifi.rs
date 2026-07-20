/**
    author: Jean-Nicolas Gosselin, Anahì Michelle Mongelos Toledo
    cip: gosj2008, mona3503
    date: 2026-07-20
**/

use anyhow::Result;
use std::time::Duration;
use std::thread::sleep;
use esp_idf_svc::wifi::{AuthMethod, ClientConfiguration, Configuration, EspWifi};

pub fn connect_wifi(wifi: &mut EspWifi<'_>) -> Result<()> {
    let wifi_ssid = dotenvy_macro::dotenv!("WIFI_SSID");
    let wifi_pass = dotenvy_macro::dotenv!("WIFI_PASS");

    wifi.set_configuration(&Configuration::Client(ClientConfiguration {
        ssid: wifi_ssid.try_into()?,
        password: wifi_pass.try_into()?,
        auth_method: AuthMethod::WPAWPA2Personal,
        ..Default::default()
    }))?;
    
    wifi.start()?;
    
    sleep(Duration::from_millis(1500));
    wifi.connect()?;
    
    while !wifi.is_connected()? {
        sleep(Duration::from_millis(100));
    }
    
    while wifi.sta_netif().get_ip_info()?.ip.is_unspecified() {
        sleep(Duration::from_millis(100));
    }
    
    Ok(())
}