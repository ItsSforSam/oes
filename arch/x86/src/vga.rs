use core::ffi::c_char;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ColorCode(pub u8);

// impl ColorCode {}
#[repr(C, packed)]
#[derive(Debug)]
struct VbeInfoBlock {
    signature: [c_char; 4],
    version: u16,
    oem_string_ptr: [u16; 2],
    capabilities: [u8; 4],
    vid_mod_ptr: [u16; 2],
    total_memory: u16,
    _reserved: [u8; 492],
}
