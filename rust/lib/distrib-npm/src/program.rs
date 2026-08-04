// This is free and unencumbered software released into the public domain.

use alloc::boxed::Box;
use core::error::Error;
use distrib_common::{Build, Utf8PathBuf};

pub const NPM_COMMAND: &str = "npm";

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct NpmProgram(Utf8PathBuf);

impl Default for NpmProgram {
    fn default() -> Self {
        Self(NPM_COMMAND.into())
    }
}

impl From<Utf8PathBuf> for NpmProgram {
    fn from(input: Utf8PathBuf) -> Self {
        Self(input)
    }
}

impl NpmProgram {}

impl Build for NpmProgram {
    fn build(&self) -> Result<(), Box<dyn Error>> {
        todo!() // TODO
    }
}
