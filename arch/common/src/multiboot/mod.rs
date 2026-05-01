//! Multiboot is a specification boot loaders
//!
//! It can allow us to retrieve info about the system via [`reader`] and give info via [`writer`]

pub mod reader;
pub(crate) mod sys;
pub mod writer;
