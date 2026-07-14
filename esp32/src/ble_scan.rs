use anyhow::Result;
use esp32_nimble::{BLEDevice, BLEScan};
use log::info;
use std::sync::mpsc;

pub fn parse_ibeacon_uuid(payload: &[u8]) -> Option<String> {
    let offset: usize = if payload.starts_with(&[0x4C, 0x00, 0x02, 0x15]) && payload.len() >= 25 {
        4
    } else if payload.starts_with(&[0x02, 0x15]) && payload.len() >= 23 {
        2
    } else {
        return None;
    };

    let u: &[u8] = &payload[offset..offset + 16];
    Some(format!(
        "{:02X}{:02X}{:02X}{:02X}-{:02X}{:02X}-{:02X}{:02X}-{:02X}{:02X}-{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}",
        u[0], u[1], u[2], u[3], u[4], u[5], u[6], u[7],
        u[8], u[9], u[10], u[11], u[12], u[13], u[14], u[15]
    ))
}

pub async fn run_ble_scan(tx: mpsc::SyncSender<String>) -> Result<()> {
    let ble_device = BLEDevice::take();
    let mut ble_scan = BLEScan::new();

    loop {
        if let Some(uuid) = ble_scan
            .active_scan(false)
            .interval(500)
            .window(30)
            .filter_duplicates(false)
            .start(&ble_device, 1000, |device, data| {
                let payload: &[u8] = data.manufacture_data()?.payload;
                let uuid = parse_ibeacon_uuid(payload)?;
                
                info!("iBeacon Detected! UUID: {}, RSSI: {} dBm", uuid, device.rssi());
                Some(uuid)
            }).await?
            {
            let _ = tx.try_send(uuid);
            }
        esp_idf_svc::hal::delay::FreeRtos::delay_ms(3000);
    }
}
