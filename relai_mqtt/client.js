require('dotenv').config();
const WebSocket = require('ws');
const mqtt = require('mqtt');

const mqttClient = mqtt.connect('mqtt://localhost');
const wsIp = process.env.WS_IP;
const ws = new WebSocket(`ws://${wsIp}:8080`);

ws.on('message', (data) => {
    const { uuid } = JSON.parse(data);
    if (uuid) {
        mqttClient.publish('esp32/uuid', uuid);
        console.log('Published:', uuid);
    }
});

ws.on('error', console.error);
mqttClient.on('error', console.error);
