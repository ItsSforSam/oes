#![no_std]
pub mod printk;
pub mod sync;

#[cfg(feature = "alloc")]
extern crate alloc;
