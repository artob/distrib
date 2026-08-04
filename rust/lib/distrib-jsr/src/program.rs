// This is free and unencumbered software released into the public domain.

use alloc::boxed::Box;
use core::error::Error;
use distrib_common::{Build, Utf8PathBuf};

pub const JSR_COMMAND: &str = "npx jsr";

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Jsr(Utf8PathBuf);

impl Default for Jsr {
    fn default() -> Self {
        Self(JSR_COMMAND.into())
    }
}

impl From<Utf8PathBuf> for Jsr {
    fn from(input: Utf8PathBuf) -> Self {
        Self(input)
    }
}

impl Jsr {}

impl Build for Jsr {
    fn build(&self) -> Result<(), Box<dyn Error>> {
        todo!() // TODO
    }
}
