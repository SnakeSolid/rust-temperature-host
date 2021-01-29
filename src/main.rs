#[macro_use]
extern crate log;

mod handler;
mod options;
mod server;
mod worker;

use crate::options::Options;
use structopt::StructOpt;

fn main() {
    env_logger::init();

    let options = Options::from_args();

    worker::start(&options);
    server::start(&options);
}
