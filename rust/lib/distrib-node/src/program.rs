// This is free and unencumbered software released into the public domain.

use alloc::boxed::Box;
use core::error::Error;
use distrib_common::{Build, Utf8PathBuf};

pub const NODE_COMMAND: &str = "node";

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct NodeProgram(Utf8PathBuf);

impl Default for NodeProgram {
    fn default() -> Self {
        Self(NODE_COMMAND.into())
    }
}

impl From<Utf8PathBuf> for NodeProgram {
    fn from(input: Utf8PathBuf) -> Self {
        Self(input)
    }
}

impl NodeProgram {}

impl Build for NodeProgram {
    fn build(&self) -> Result<(), Box<dyn Error>> {
        todo!() // TODO
    }
}
