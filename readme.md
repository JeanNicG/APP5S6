# IoT ibeacon scanner

## esp32

create a .env with
```
WIFI_SSID=
WIFI_PASS=
```
Run the project
```
cd esp32
cargo run
```

## api

```
cd api
node api.js
```

## relai_mqtt

create a .env with
```
WS_IP=
```
Run the project
```
cd relai_mqtt
node client.js
```

## server mqtt
```
sudo systemctl enable --now mosquitto
sudo systemctl status mosquitto
mosquitto_sub -t "esp32/uuid" -d
```

## frontend

``` 
cd frontend
trunk serve
```