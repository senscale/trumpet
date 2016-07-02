use std::io;

pub struct HadoopConf;

impl HadoopConf {

  pub fn new() -> HadoopConf {
    HadoopConf
  }

}

#[derive(Debug)]
pub enum HadoopError {
  ConnectionError(io::Error),
}

impl From<io::Error> for HadoopError {
  fn from(err: io::Error) -> HadoopError {
    HadoopError::ConnectionError(err)
  }
}
