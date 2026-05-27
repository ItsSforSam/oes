use crate::common::ascii::AStr;

pub struct CommandLine {
    // inner: Option<Vec<Vec<u8>>>,
}

pub struct CmdLine<'a> {
    inner: spin::Once<&'a crate::common::ascii::AStr>,
}

impl Default for CmdLine<'a> {
    /// Constructs a empty commandline
    fn default() -> Self {
        CmdLine {
            inner: spin::Once::initialized(&'a AStr::default()),
        }
    }
}
