// This is free and unencumbered software released into the public domain.

use alloc::boxed::Box;
use core::error::Error;
use distrib_common::{Build, Utf8PathBuf};

pub const GEM_COMMAND: &str = "gem";

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct GemProgram(Utf8PathBuf);

impl Default for GemProgram {
    fn default() -> Self {
        Self(GEM_COMMAND.into())
    }
}

impl From<Utf8PathBuf> for GemProgram {
    fn from(input: Utf8PathBuf) -> Self {
        Self(input)
    }
}

impl GemProgram {}

impl Build for GemProgram {
    fn build(&self) -> Result<(), Box<dyn Error>> {
        todo!() // TODO
    }
}
