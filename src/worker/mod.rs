use crate::options::Options;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Result as IoResult;
use std::io::Write;
use std::path::PathBuf;
use std::thread;
use std::thread::Builder;
use std::time::Duration;
use time::OffsetDateTime;

#[derive(Debug)]
pub struct Worker {
    serial_port: String,
    data_path: PathBuf,
}

const ERROR_TIMEOUT: u64 = 10;
const SERIAL_TIMEOUT: u64 = 24 * 60 * 60;
const SERIAL_BAUD_RATE: u32 = 9_600;

impl Worker {
    #[allow(clippy::needless_pass_by_value)]
    fn new(serial_port: String, data_path: PathBuf) -> Worker {
        Worker {
            serial_port,
            data_path,
        }
    }

    fn start(self) {
        loop {
            info!("Connecting to `{}`", self.serial_port);

            let port = serialport::new(&self.serial_port, SERIAL_BAUD_RATE)
                .timeout(Duration::from_secs(SERIAL_TIMEOUT))
                .open()
                .expect("Failed to open serial port");
            let reader = BufReader::new(port);

            info!("Start reading");

            for line in reader.lines() {
                let line = match line {
                    Ok(line) if line == "CPU;Humidity;Temperature" => continue,
                    Ok(line) => line,
                    Err(err) => {
                        warn!("Failed to read from source - {}", err);

                        break;
                    }
                };

                let now = OffsetDateTime::now_utc().unix_timestamp();

                if let Err(err) = self.with_data(|mut file| writeln!(file, "{};{}", now, line)) {
                    warn!("Failed to write to source - {}", err);
                }
            }

            warn!("Communication complete");

            thread::sleep(Duration::from_secs(ERROR_TIMEOUT));

            warn!("Reconnection");
        }
    }

    fn with_data<F>(&self, callback: F) -> IoResult<()>
    where
        F: FnOnce(File) -> IoResult<()>,
    {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.data_path)?;

        self.ensure_header(&mut file)?;

        callback(file)?;

        Ok(())
    }

    fn ensure_header(&self, file: &mut File) -> IoResult<()> {
        let metadata = file.metadata()?;
        let length = metadata.len();

        if length == 0 {
            info!("Data file is empty. Adding file header");

            writeln!(file, "DateTime;CPU;Humidity;Temperature")?;
        }

        Ok(())
    }
}

pub fn start(options: &Options) {
    let serial_port = options.serial_port().to_string();
    let data_path = options.data_path().to_path_buf();

    if let Err(err) = Builder::new()
        .name("data worker".to_string())
        .spawn(move || Worker::new(serial_port, data_path).start())
    {
        warn!("Failed to start data worker - {}", err);
    }
}
