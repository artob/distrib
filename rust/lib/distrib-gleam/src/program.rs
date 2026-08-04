// This is free and unencumbered software released into the public domain.

use alloc::boxed::Box;
use core::error::Error;
use distrib_common::{Build, Utf8PathBuf};

pub const GLEAM_COMMAND: &str = "gleam";

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct GleamProgram(Utf8PathBuf);

impl Default for GleamProgram {
    fn default() -> Self {
        Self(GLEAM_COMMAND.into())
    }
}

impl From<Utf8PathBuf> for GleamProgram {
    fn from(input: Utf8PathBuf) -> Self {
        Self(input)
    }
}

impl GleamProgram {}

impl Build for GleamProgram {
    fn build(&self) -> Result<(), Box<dyn Error>> {
        todo!() // TODO
    }
}
