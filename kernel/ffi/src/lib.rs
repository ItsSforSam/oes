//! Foreign function interface (FFI) types.
//!
//! Provides mappings to C primitives to Rust
#![no_std]

macro_rules! alias {
    ($(
        $(#[$attr:meta])*
        pub type $name:ident = $type:ty;

    )*
    ) => {$(

        $(#[$attr])*
        #[allow(non_camel_case_types,missing_docs)] // Following the standard set in part with core::ffi
        pub type $name = $type;

        // We don't want different sized types and break something
        const _:() = assert!(

            ::core::mem::size_of::<$name>() == ::core::mem::size_of::<::core::ffi::$name>()
        );
    )*
    };
}

alias! {
    // we use `-fsigned-char` if we use any C code. So it's always
    pub type c_char = i8;
    pub type c_schar = i8;
    pub type c_uchar = u8;

    pub type c_short = i16;
    pub type c_ushort = u16;

    pub type c_int = i32;
    pub type c_uint = i32;

    pub type c_long  = i64;
    pub type c_ulong = u64;

    // Required to be at least 64 bits, and at least the size of
    // a long. We cannot use 128 bit types in the kernel (cause less supported and expensive)
    pub type c_longlong = i64;
    pub type c_ulonglong = u64;

}
#[doc(inline)]
pub use core::ffi::CStr;
#[doc(inline)]
pub use core::ffi::c_void;
