use log::info;
use std::sync::mpsc;
use std::net::TcpListener;
use tungstenite::{accept, Message};
use serde_json::json;

pub fn run_websocket_server(rx: mpsc::Receiver<String>) {
    let ws_server = TcpListener::bind("0.0.0.0:8080").expect("Failed to bind to port 8080");
    info!("ESP32 WebSocket server listening on ws://0.0.0.0:8080");

    for stream_result in ws_server.incoming() {
        let stream = match stream_result {
            Ok(s) => s,
            Err(_) => continue,
        };

        let mut socket = match accept(stream) {
            Ok(s) => s,
            Err(_) => continue,
        };

        info!("Node.js client connected!");
        
        // clear any old queued messages
        while let Ok(_) = rx.try_recv() {}

        loop {
            let uuid = match rx.recv() {
                Ok(u) => u,
                Err(_) => break,
            };

            let payload = json!({ "uuid": uuid }).to_string();
            
            if socket.send(Message::Text(payload)).is_err() {
                info!("Client disconnected");
                break;
            }
        }
    }
}