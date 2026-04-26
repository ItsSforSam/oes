//! Multiboot is a specification boot loaders
//! follow and how a OS can ask things of bootloader
//! and of the like

use core::ptr::null;

use bytemuck::Zeroable;
#[repr(C)]
#[derive(Debug, Zeroable, PartialEq, Eq)]
struct Tag {
    pub type_: u16,
    pub flags: u16,
    pub size: u32,
}
impl Tag {
    const fn terminator() -> Tag {
        Tag {
            type_: 0,
            flags: 0,
            size: 8,
        }
    }
}

impl Default for Tag {
    /// Construct the terminating tag
    /// See [`Tag::terminator`] for more details
    fn default() -> Self {
        Tag::terminator()
    }
}

#[repr(C)]
#[derive(Debug, Zeroable)]
struct Info {
    pub total_size: u32,
    /// Always set to zero and should be ignored
    _reserved: u32,
}
#[repr(C)]
#[derive(Debug, Zeroable)]
struct Header {
    /// A value that should be `0xE85250D6`
    magic: u32,
    architecture: u32,
    header_length: u32,
    checksum: u32,
}

impl Header {}

// SAFETY: handled by bootloader
// unsafe extern "C" {
//     safe static mut header: *const Header;
// }
