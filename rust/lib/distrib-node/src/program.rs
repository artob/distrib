// This is free and unencumbered software released into the public domain.

use alloc::{boxed::Box, string::String, vec::Vec};
use core::error::Error;
use distrib_common::{Build, Clean, Tool, Utf8PathBuf};
use std::process::Command;

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

impl<T> From<&T> for NodeProgram
where
    T: Clone + Into<Self>,
{
    fn from(t: &T) -> Self {
        t.clone().into()
    }
}

impl From<NodeProgram> for Command {
    fn from(input: NodeProgram) -> Self {
        Command::new(input.0)
    }
}

impl NodeProgram {}
