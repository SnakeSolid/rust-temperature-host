## Temperature and Humidity Sensor Host

Opens connection to Bluetooth COMM port and store all received data to given file. Every row will be stored with current timestamp as first row. If connection will be lost reader will try to reconnect after 10 seconds.

### Command Line Arguments

* `-a`, `--address` -  Address to to start HTTP server on (by default - localhost);
* `-d`, `--data-path` -  Path to store data from DHT22 (by )default - data.csv);
* `-p`, `--port` -  Web server will listen on this port (by default - 8080);
* `-s`, `--serial-port` - Path to serial port with sensor data (by default: /dev/rfcomm0)
