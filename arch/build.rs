use core::fmt;
use std::{env, str::FromStr};
fn main() {
    let arch = env::var("CARGO_CFG_TARGET_ARCH").expect("Cannot determine architecture");
    let arch = TargetArchitecture::from_str(&arch).unwrap();
    // If we are apart of libtest, prevent our special linker script, as that
    // can break compiling
    if std::env::var("CARGO_FEATURE_LIBTEST").is_err() {
        // this will be where Cargo.toml and build.rs is located
        // We do this as rustc may be executing in a different directory
        let dir = std::env::current_dir().unwrap();
        println!("cargo::rustc-link-arg=-T{}/{arch}/ld-oes.ld", dir.display());
    }
}
/// Represents all available targets for The Odyssey Entertainment System.
#[expect(nonstandard_style, reason = "Doesn't read well in context")]
#[derive(Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum TargetArchitecture {
    /// Represents x86 and x86_64
    x86,
}
impl Default for TargetArchitecture {
    /// The = is [`x86`] currently, besides that being our only supported target,
    /// this is due to it being the most supported target, but that can always change, so don't rely on
    /// it always being [`x86`]. If you need it to be [`x86`], just use it explicitly
    ///
    /// # Example
    ///
    /// Using a sain default target
    /// ```
    /// # fn get_current_target() -> Option<TargetArchitecture> {None}
    ///
    /// let current_target: TargetArchitecture = get_current_target().unwrap_or_default();
    /// ```
    /// What not to do
    /// ```no_run
    /// let definitely_x86:TargetArchitecture = Default::default();
    /// // This doesn't panic, but can start panicking with little to no notice
    /// assert!(definitely_x86 == TargetArchitecture::x86 )
    /// ```
    ///
    /// [`x86`]: TargetArchitecture::x86
    fn default() -> Self {
        TargetArchitecture::x86
    }
}
#[derive(Debug)]
pub struct UnsupportedArchitecture(String);
impl core::fmt::Display for UnsupportedArchitecture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unsupported Architecture `{}`", self.0)
    }
}
impl std::error::Error for UnsupportedArchitecture {}
impl std::str::FromStr for TargetArchitecture {
    type Err = UnsupportedArchitecture;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "x86" | "x86_64" => Ok(TargetArchitecture::x86),

            other => Err(UnsupportedArchitecture(other.to_owned())),
        }
    }
}
impl TargetArchitecture {
    pub const fn as_str(&self) -> &str {
        match self {
            TargetArchitecture::x86 => "x86",
        }
    }
}
impl fmt::Display for TargetArchitecture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
