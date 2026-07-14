const express = require('express');
const router = express.Router();
const coap = require('coap');

// Helper to wrap the CoAP request in a Promise
const sendCoapRequest = (host, action) => {
    return new Promise((resolve, reject) => {
        const req = coap.request({ host, pathname: `/led/${action}`, method: 'POST' });
        
        // Timeout to ensure we don't hang infinitely if the ESP32 is unreachable
        const timer = setTimeout(() => {
            reject(new Error("Timeout: ESP32 took too long to respond. Check the IP address."));
        }, 5000);

        req.on('response', (res) => {
            clearTimeout(timer);
            resolve(res.code);
        });

        req.on('error', (err) => {
            clearTimeout(timer);
            reject(err);
        });
        
        req.on('timeout', (err) => {
            clearTimeout(timer);
            reject(new Error("CoAP Request Timeout"));
        });

        req.end();
    });
};

// Route to handle LED control
// Expects JSON body: { "deviceIp": "192.168.1.100", "action": "on" | "off" }
router.post('/', async (req, res) => {
    const { deviceIp, action } = req.body;

    if (!deviceIp || !['on', 'off'].includes(action)) {
        return res.status(400).json({ error: "Valid 'deviceIp' and 'action' ('on' or 'off') are required" });
    }

    try {
        const responseCode = await sendCoapRequest(deviceIp, action);
        
        if (responseCode === '2.04') {
            res.json({ success: true, message: `LED turned ${action} successfully` });
        } else {
            res.status(500).json({ success: false, error: `Device returned error code: ${responseCode}` });
        }
    } catch (error) {
        console.error("CoAP request failed:", error);
        res.status(500).json({ success: false, error: error.message || "Failed to communicate with device" });
    }
});

module.exports = router;
