// This is free and unencumbered software released into the public domain.

use alloc::boxed::Box;
use core::error::Error;
use distrib_common::{Build, Utf8PathBuf};

pub const DART_COMMAND: &str = "dart";

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct DartProgram(Utf8PathBuf);

impl Default for DartProgram {
    fn default() -> Self {
        Self(DART_COMMAND.into())
    }
}

impl From<Utf8PathBuf> for DartProgram {
    fn from(input: Utf8PathBuf) -> Self {
        Self(input)
    }
}

impl DartProgram {}

impl Build for DartProgram {
    fn build(&self) -> Result<(), Box<dyn Error>> {
        todo!() // TODO
    }
}
