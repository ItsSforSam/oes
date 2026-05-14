pub struct Device {
    // initial_name: Box<str>,
}

pub struct Driver {
    name: &'static str,
}
impl Driver {
    pub const fn new(name: &'static str) -> Driver {
        Driver {
            name,
        }
    }
}
// pub trait Driver {}
