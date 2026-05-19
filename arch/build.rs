use core::fmt;
use std::{env, str::FromStr};
fn main() {
    let arch = env::var("CARGO_CFG_TARGET_ARCH").expect("Cannot determine architecture");
    let arch = TargetArchitecture::from_str(&arch).unwrap();
    println!("cargo::rustc-link-arg=-T{arch}/ld-oes.ld");
}
#[expect(nonstandard_style, reason = "Doesn't read well in context")]
pub enum TargetArchitecture {
    /// Represents x86 and x86_64
    x86,
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
