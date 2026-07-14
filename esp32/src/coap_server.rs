use anyhow::Result;
use coap_lite::{CoapRequest, MessageClass, MessageType, Packet, RequestType, ResponseType};
use esp_idf_svc::hal::gpio::{Output, PinDriver};
use std::net::UdpSocket;
use std::sync::{Arc, Mutex};

pub fn run_coap_server(led: Arc<Mutex<PinDriver<'static, Output>>>) -> Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:5683")?;
    let mut buf = [0; 1024];

    loop {
        let Ok((size, src)) = socket.recv_from(&mut buf) else { continue };
        let Ok(packet) = Packet::from_bytes(&buf[..size]) else { continue };
        
        let mut request = CoapRequest::from_packet(packet, src);
        let path = request.get_path();
        log::info!("Received CoAP request from {} with path: {}", src, path);
        
        let response_code = match (request.message.header.code, path.as_str()) {
            (MessageClass::Request(RequestType::Post), "led/on") => {
                let _ = led.lock().unwrap().set_high();
                ResponseType::Changed
            }
            (MessageClass::Request(RequestType::Post), "led/off") => {
                let _ = led.lock().unwrap().set_low();
                ResponseType::Changed
            },
            _ => ResponseType::NotFound,
        };

        let Some(ref mut response) = request.response else { continue };
        response.message.header.set_type(MessageType::Acknowledgement);
        response.message.header.code = MessageClass::Response(response_code);
        
        let Ok(bytes) = response.message.to_bytes() else { continue };
        let _ = socket.send_to(&bytes, src);
    }
}
