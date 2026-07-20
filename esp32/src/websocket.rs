/**
    author: Jean-Nicolas Gosselin, Anahì Michelle Mongelos Toledo
    cip: gosj2008, mona3503
    date: 2026-07-20
**/

use log::info;
use std::sync::mpsc;
use std::net::TcpListener;
use tungstenite::{accept, Message};
use serde_json::json;

pub fn run_websocket_server(rx: mpsc::Receiver<String>) {
    let ws_server = TcpListener::bind("0.0.0.0:8080").expect("Failed to bind to port 8080");
    info!("ESP32 WebSocket server listening on ws://0.0.0.0:8080");

    for stream_result in ws_server.incoming() {
        let Ok(stream) = stream_result else { continue };
        let Ok(mut socket) = accept(stream) else { continue };
        info!("Node.js client connected!");
        
        // clear any old queued messages
        while let Ok(_) = rx.try_recv() {}

        loop {
            let Ok(uuid) = rx.recv() else { break };
            let payload = json!({ "uuid": uuid }).to_string();
            
            if socket.send(Message::Text(payload)).is_err() {
                info!("Client disconnected");
                break;
            }
        }
    }
}