use core::fmt;
#[derive(Debug)]
pub struct DeviceError {}

impl core::error::Error for DeviceError {}

impl fmt::Display for DeviceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}
