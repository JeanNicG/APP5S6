/**
    author: Jean-Nicolas Gosselin, Anahì Michelle Mongelos Toledo
    cip: gosj2008, mona3503
    date: 2026-07-20
**/

require('dotenv').config();
const WebSocket = require('ws');
const mqtt = require('mqtt');

const mqttBrokerUrl = process.env.MQTT_BROKER_URL;
const mqttClient = mqtt.connect(mqttBrokerUrl);
const wsIp = process.env.WS_IP;
const ws = new WebSocket(`ws://${wsIp}:8080`);

ws.on('message', (data) => {
    const { uuid } = JSON.parse(data);
    if (uuid) {
        mqttClient.publish('esp32/uuid', uuid);
        console.log('Published:', uuid);
    }
});

ws.on('error', (err) => {
    console.error(err);
    process.exit(1);
});

ws.on('close', () => process.exit(1));

mqttClient.on('error', console.error);
