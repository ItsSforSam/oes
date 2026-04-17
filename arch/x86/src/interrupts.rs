use x86_64::structures::idt;

#[repr(transparent)]
pub struct Idt {
    inner: idt::InterruptDescriptorTable,
}

impl Idt {
    /// This constructs the Interrupt Descriptor Table and places all the necessary handlers into
    pub fn new() -> Idt {
        let mut inter = idt::InterruptDescriptorTable::new();

        Idt {
            inner: inter,
        }
    }
    /// Constructs a interrupt table in place but with all the entries empty
    ///
    pub const fn empty() -> Idt {
        Idt {
            inner: idt::InterruptDescriptorTable::new(),
        }
    }
    /// Loads the current interrupt table into what is used
    ///
    /// # SAFETY
    /// This should simply not be called too much as
    pub unsafe fn load(&'static self) {
        self.inner.load();
    }
}
impl AsRef<idt::InterruptDescriptorTable> for Idt {
    fn as_ref(&self) -> &idt::InterruptDescriptorTable {
        &self.inner
    }
}
impl AsMut<idt::InterruptDescriptorTable> for Idt {
    /// This allows you to modify any of the internal structure, if needed
    fn as_mut(&mut self) -> &mut idt::InterruptDescriptorTable {
        &mut self.inner
    }
}
