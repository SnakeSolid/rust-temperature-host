use crate::options::Options;
use iron::middleware::Handler;
use iron::response::Response;
use iron::status::Status;
use iron::IronResult;
use iron::Request as IronRequest;
use iron::Response as IronResponse;
use std::fs::File;
use std::path::PathBuf;

#[derive(Debug)]
pub struct DataHandler {
    data_path: PathBuf,
}

impl DataHandler {
    pub fn new(options: &Options) -> DataHandler {
        DataHandler {
            data_path: options.data_path().to_path_buf(),
        }
    }
}

impl Handler for DataHandler {
    fn handle(&self, _request: &mut IronRequest) -> IronResult<IronResponse> {
        if let Ok(file) = File::open(&self.data_path) {
            Ok(Response::with((Status::Ok, file)))
        } else {
            Ok(Response::with(Status::InternalServerError))
        }
    }
}
