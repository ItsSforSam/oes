use core::error::Error;
use core::fmt;
#[derive(Debug)]
pub enum DeviceInitError {
    /// Indicates that the driver is already loaded
    ///
    /// The string to pass in is the driver's identifier
    AlreadyInitialized(&'static str),
    /// Some other error relating to the driver
    Other(DeviceError),
}

impl Error for DeviceInitError {}

impl fmt::Display for DeviceInitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use DeviceInitError::*;
        match self {
            AlreadyInitialized(driver_name) => {
                write!(f, "{driver_name} is already initialized and loaded")
            }
            Other(v) => fmt::Display::fmt(v, f),
        }
    }
}

#[derive(Debug)]
pub enum DeviceError {}

impl core::error::Error for DeviceError {}

impl fmt::Display for DeviceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}
