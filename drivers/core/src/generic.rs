//! Generic drivers to fall back too

#[cfg(feature = "uefi")]
mod uefi {

    // pub struct Debug
}

pub struct DebugPort(());

impl DebugPort {
    pub fn new() -> Self {
        DebugPort(())
    }
}

pub struct GraphicsPort {}

// #[cfg(feature = "uefi")]
// pub use crate::generic::uefi::*;
