use std::path::Path;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "temperature-host")]
pub struct Options {
    #[structopt(
        short = "s",
        long = "serial-port",
        name = "SERIAL_PORT",
        help = "Use SERIAL_PORT as serial port (to read sensor data)",
        default_value = "/dev/rfcomm0"
    )]
    serial_port: String,

    #[structopt(
        short = "d",
        long = "data-path",
        name = "DATA_PATH",
        help = "Use DATA_PATH as output data file",
        default_value = "data.csv",
        parse(from_os_str)
    )]
    data_path: PathBuf,

    #[structopt(
        short = "a",
        long = "address",
        name = "ADDR",
        help = "Listen on given address",
        default_value = "localhost"
    )]
    address: String,

    #[structopt(
        short = "p",
        long = "port",
        name = "PORT",
        help = "Listen on given port",
        default_value = "8080"
    )]
    port: u16,
}

impl Options {
    pub fn serial_port(&self) -> &str {
        &self.serial_port
    }

    pub fn data_path(&self) -> &Path {
        &self.data_path
    }

    pub fn address(&self) -> &str {
        &self.address
    }

    pub fn port(&self) -> u16 {
        self.port
    }
}
