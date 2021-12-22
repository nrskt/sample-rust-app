use ports::PortError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApplicationError {
    #[error("InterfaceError: {0}")]
    InterfaceError(#[from] PortError),
}
