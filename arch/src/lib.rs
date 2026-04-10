// This re-exports all the architecture specific
#![no_std]

#[cfg(target_arch = "x86_64")]
pub use oes_arch_x86 as x86;


pub mod common{
    #[expect(unused_imports)]
    #[doc(inline)]
    pub use ::oes_arch_common::*;
}