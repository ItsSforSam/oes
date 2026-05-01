use core::{fmt, marker::PhantomPinned, num::NonZero, ops::Div};

use bytemuck::Zeroable;
/// Used with [`TagType`]'s [`TryFrom`] with an invalid
#[derive(Debug)]
pub struct InvalidTagType(u32);

impl fmt::Display for InvalidTagType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid Tag type of {}", self.0)
    }
}

impl core::error::Error for InvalidTagType {}

/// A Tag that the bootloadr
#[repr(C, align(8))]
#[derive(Debug, Zeroable, PartialEq, Eq)]
struct Tag {
    /// The internal type
    ///
    /// This isn't using [`TagType`] internally as
    /// a non compliant bootloader can pass arbitrary tags
    ///
    type_: u32,
    // pub flags: u16,
    size: u32,
    _pin: PhantomPinned,
}
impl Tag {
    pub const fn new(type_: TagType, size: u32) -> Tag {
        Tag {
            type_: type_.into(),
            // flags,
            size,
            _pin: PhantomPinned,
        }
    }
    pub const fn terminator() -> Tag {
        Tag {
            type_: 0,
            // flags: 0,
            size: 8,
            _pin: PhantomPinned,
        }
    }
}
/// A Type to be used with a [`Info`].
///
/// This lists all the types in the [multiboot2 spec]
///
/// [multiboot2 spec]: https://www.gnu.org/software/grub/manual/multiboot2/multiboot.html#Boot-information-format
#[repr(u32)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[non_exhaustive]
pub enum TagType {
    End = 0,
    Cmdline = 1,
    BootLoaderName = 2,
    Module,
    BasicMeminfo,
    Bootdev,
    Mmap,
    Vbe,
    Framebuffer,
    ElfSections,
    Apm,
    Efi32,
    Efi64,
    Smbios,
    AcpiV1,
    AcpiV2,
    Network,
    EfiMmap,
    EfiBs,
    Efi32Ih,
    Efi64Ih,
    LoadBaseAddr,
}

impl TryFrom<u32> for TagType {
    type Error = InvalidTagType;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        use TagType::*;

        let ret = match value {
            0 => Ok(End),
            1 => Ok(Cmdline),
            2 => Ok(BootLoaderName),
            3 => Ok(Module),
            4 => Ok(BasicMeminfo),
            5 => Ok(Bootdev),
            6 => Ok(Mmap),
            7 => Ok(Vbe),
            8 => Ok(Framebuffer),
            9 => Ok(ElfSections),
            10 => Ok(Apm),
            11 => Ok(Efi32),
            12 => Ok(Efi64),
            13 => Ok(Smbios),
            14 => Ok(AcpiV1),
            15 => Ok(AcpiV2),
            16 => Ok(Network),
            17 => Ok(EfiMmap),
            18 => Ok(EfiBs),
            19 => Ok(Efi32Ih),
            20 => Ok(Efi64Ih),
            21 => Ok(LoadBaseAddr),
            value => Err(InvalidTagType(value)),
        };
        #[cfg(debug_assertions)]
        {
            match ret {
                Ok(v) => assert!(v == value),
                Err(_) => { /* Nothing */ }
            }
        }
        ret
    }
}
impl Default for Tag {
    /// Construct the terminating tag
    /// See [`Tag::terminator`] for more details
    fn default() -> Self {
        Tag::terminator()
    }
}
impl PartialEq<u32> for TagType {
    fn eq(&self, other: &u32) -> bool {
        (*self as u32) == *other
    }
}
impl Eq<u32> for TagType {}
#[repr(C, align(8))]
#[derive(Debug)]
struct Info {
    /// The total size, in bytes
    total_size: u32,
    /// Always set to zero and should be ignored
    _reserved: u32,
    _pin: PhantomPinned,
}
impl Info {
    /// Constructs a new [`Info`]
    pub const fn new(total_size: u32) -> Self {
        Info {
            total_size,
            _reserved: 0,
            _pin: PhantomPinned,
        }
    }
    pub fn get_tags_mut(&mut self) -> &mut [Tag] {
        let r = &raw mut *self;

        let t: *mut Tag = r.cast();
        use core::slice::from_raw_parts_mut;
        // #[expect(clippy::arithmetic_side_effects)]
        let len =
        // SAFETY: 
        unsafe { (self.total_size as usize).checked_div(size_of::<Info>()).unwrap_unchecked() };
        let slice = unsafe { from_raw_parts_mut(t, len) };
        &mut slice[1..]
    }
}

/// The header for Multiboot2
#[repr(C, align(8))]
#[derive(Debug)]
struct Header {
    /// A value that should be `0xE85250D6`
    magic: u32,
    architecture: u32,
    header_length: u32,
    /// Added value of [`magic`],[`architecture`],[`header_length`], must have a sum of 0
    ///
    ///
    ///
    /// [`magic`]: Header::magic
    /// [`architecture`]: Header::architecture
    /// [`header_length`]: Header::header_length
    checksum: u32,

    tags: crate::IncompleteArrayField<Tag>,
}

impl Header {
    /// The magic value
    pub fn magic(&self) -> u32 {
        self.magic
    }
}
// SAFETY: handled by bootloader
// unsafe extern "C" {
//     safe static mut header: *const Header;
// }
// A trait
// unsafe trait TagType<T> {
//     /// The number that is ise
//     const TYPE_TAG: u32;
//     ///
//     fn into_type(self: *const Self) -> Option<T>;

//     /// Each Tag has it's own type id
//     fn type_id(&self) -> u32;
// }
#[repr(C)]
pub struct BootloaderName {
    /// Always `2`
    type_: u32,
    size: u32,
    /// A C-String that is zero-terminated and encoded as UTF-8.
    /// It's a invarient if otherwise
    string: crate::IncompleteArrayField<i8>,
    _pin: PhantomPinned,
}
impl BootloaderName {
    /// Get the name of the bootloader
    pub fn as_str(&self) -> &str {
        // SAFETY: Struct invariant if this is not upheld
        let s = unsafe { core::ffi::CStr::from_ptr(self.string.as_ptr()) };
        // This can probably be .unwrap_unchecked, BUT we should check it to ensure
        s.to_str().expect("BootloaderName not UTF-8 encoded")
    }
}
#[repr(C)]
pub struct BootCmdLine {
    type_: u32,
    size: u32,
    /// A C-String that is zero-terminated and encoded as UTF-8.
    /// It's a invarient if otherwise
    string: *const i8,
    _pin: PhantomPinned,
}
