use core::fmt;
#[derive(Debug)]
pub enum DeviceInitError {
    AlreadyInitialized,
    /// Some other error relating to the driver
    Other(DeviceError),
}

#[derive(Debug)]
pub enum DeviceError {}

impl core::error::Error for DeviceError {}

impl fmt::Display for DeviceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}
