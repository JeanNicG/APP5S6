use gloo_net::http::Request;
use serde::Serialize;

#[derive(Serialize)]
struct ControlBody<'a> {
    #[serde(rename = "deviceIp")]
    device_ip: &'a str,
    action: &'a str,
}

pub async fn fetch_archive() -> String {
    match Request::get("http://localhost:3000/archive").send().await {
        Ok(res) => res.text().await.unwrap_or_else(|_| "Failed to parse text".into()),
        Err(e) => format!("Error fetching data: {}", e),
    }
}

pub async fn set_device_status(action: &str) {
    let body = ControlBody {
        device_ip: "10.159.131.110",
        action,
    };
    
    let _ = Request::post("http://localhost:3000/control")
        .json(&body)
        .expect("Failed to serialize body")
        .send()
        .await;
}
