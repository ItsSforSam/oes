// This re-exports all the architecture specific
#![no_std]

cfg_if::cfg_if! {
    if #[cfg(target_arch = "x86_64")]{
        pub use oes_arch_x86 as x86;
        pub use oes_arch_x86 as current;
    }else {
        compile_error!("Architecture ",env!("CARGO_CFG_TARGET_ARCH"), "not supported.");
    }
}

pub mod common {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use ::oes_arch_common::*;
}
