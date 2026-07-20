/**
    author: Jean-Nicolas Gosselin, Anahì Michelle Mongelos Toledo
    cip: gosj2008, mona3503
    date: 2026-07-20
**/

const router = require('express').Router();
const coap = require('coap');

router.post('/', (req, res) => {
    const { deviceIp, action } = req.body;
    if (!deviceIp || !['on', 'off'].includes(action)) return res.status(400).send("Invalid input");

    const coapReq = coap.request({ host: deviceIp, pathname: `/led/${action}`, method: 'POST' });
    
    coapReq.on('response', (coapRes) => res.json({ success: coapRes.code === '2.04' }));
    coapReq.on('error', (err) => res.status(500).send(err.message));
    coapReq.on('timeout', () => res.status(504).send("Timeout"));
    
    coapReq.end();
});

module.exports = router;