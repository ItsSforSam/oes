use core::num;

#[derive(Debug)]
pub struct Syscall {
    call_number: u32,
    name: Box<str>,
    type_: SyscallType,
}
/// The type of syscall
///
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub enum SyscallType {
    /// A standard syscall
    ///
    /// This **doesn't** mean it follows [POSIX] or any defined standard.
    /// (Although most of more standard POSIX syscalls are probably included)
    ///
    /// This means that these are most
    ///
    /// [POSIX]: https://en.wikipedia.org/wiki/POSIX
    STD,
    /// Recognized `-ENOSYS`
    Unimplemented,
    /// An "extension", not a standard
    Ext,
}
pub struct InvalidSyscallTypeError(String);
impl InvalidSyscallTypeError {
    /// Get a reference to the attempted string
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
impl std::fmt::Display for InvalidSyscallTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid Syscall Type: `{}`", self.0)
    }
}
impl core::fmt::Debug for InvalidSyscallTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("InvalidSyscallTypeError").finish_non_exhaustive()
    }
}
impl std::str::FromStr for SyscallType {
    type Err = InvalidSyscallTypeError;
    /// Parse the type from string
    fn from_str(s: &str) -> Result<Self, InvalidSyscallTypeError> {
        // We store upper and a reference so we can use string literals
        // and stop committing to the heap unnecessary as .to_uppercase already makes
        // a new string,
        let upper = s.to_uppercase();
        let checked = &*upper;
        match checked {
            "STD" => Ok(SyscallType::STD),
            "UNIMPL" => Ok(SyscallType::Unimplemented),
            "EXT" => Ok(SyscallType::Ext),
            _ => Err(InvalidSyscallTypeError(upper)),
        }
    }
}
impl Syscall {
    /// Constructs a [`Syscall`]
    ///
    /// # Panics
    /// if `number` is 0. If you want syscall of 0, use [`Syscall::bare()`]
    #[must_use]
    pub fn new(number: u32, name: &str, type_: SyscallType) -> Syscall {
        assert!(
            number != 0,
            "Number being zero is likely an error. If you want the bare syscall call "
        );
        Syscall {
            call_number: number,
            name: name.into(),
            type_,
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    /// Returns the [`SyscallType`]
    #[must_use]
    pub fn get_type(&self) -> SyscallType {
        self.type_.clone()
    }
    /// Construct a very [`bare`] syscall
    ///
    /// This will not corelate to any valid Syscall number with
    /// the number of 0, Which always returns `-ENOSYS`
    pub fn bare() -> Syscall {
        Syscall {
            call_number: 0,
            name: Box::from("Bare"),
            type_: SyscallType::Unimplemented,
        }
    }
}
impl Default for Syscall {
    /// Constructs the [`bare`]
    ///
    /// [`bare`]: Syscall::bare
    fn default() -> Self {
        Syscall::bare()
    }
}
#[derive(Debug)]
pub enum ParsingError {
    // Serde(Box<dyn std::error::Error>),
    IO(std::io::Error),
}
impl std::fmt::Display for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParsingError::IO(error) => write!(f, "IO Fail syscall table: {}", error),
        }
    }
}

impl std::error::Error for ParsingError {}
impl From<std::io::Error> for ParsingError {
    fn from(value: std::io::Error) -> Self {
        ParsingError::IO(value)
    }
}
// -- DESERIALIZING --
pub fn parse_file<F: AsRef<std::path::Path>>(f: F) -> Result<Vec<Syscall>, ParsingError> {
    let file = std::fs::OpenOptions::new().read(true).write(false).open(f)?;
    todo!()
    // file.
}
