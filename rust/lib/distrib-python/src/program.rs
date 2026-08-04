// This is free and unencumbered software released into the public domain.

use alloc::boxed::Box;
use core::error::Error;
use distrib_common::{Build, Utf8PathBuf};

pub const PIP_COMMAND: &str = "pip";

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct PipProgram(Utf8PathBuf);

impl Default for PipProgram {
    fn default() -> Self {
        Self(PIP_COMMAND.into())
    }
}

impl From<Utf8PathBuf> for PipProgram {
    fn from(input: Utf8PathBuf) -> Self {
        Self(input)
    }
}

impl PipProgram {}

impl Build for PipProgram {
    fn build(&self) -> Result<(), Box<dyn Error>> {
        todo!() // TODO
    }
}
