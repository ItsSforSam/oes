//! Provides the [`println!`]
use core::fmt;

use oes_drivers_core::Write;

/// internal
#[doc(hidden)]
pub fn _print(msg: fmt::Arguments<'_>) {}

#[macro_export]
macro_rules! print {
    ($($args:tt)*) => {$crate::printk::_print(::core::format_args!($($arg)*))};
}
#[macro_export]
macro_rules! println {
    () => {$crate::print!("\n")};
    ($($arg:tt)*) => {$crate::print!("{}\n", format_args!($($arg)*))};
}

struct no_op;

// impl fmt::Write for no_op{
//     fn
// }
