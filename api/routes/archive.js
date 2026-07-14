const router = require('express').Router();
const fs = require('fs');
const mqtt = require('mqtt').connect('mqtt://localhost');

let uuids = fs.existsSync('uuid.json') ? JSON.parse(fs.readFileSync('uuid.json')) : [];

mqtt.on('connect', () => mqtt.subscribe('esp32/uuid'));
mqtt.on('message', (t, m) => fs.writeFileSync('uuid.json', JSON.stringify(uuids = [...uuids, m.toString()], null, 2)));

router.get('/', (req, res) => res.json(uuids));

module.exports = router;