// This is free and unencumbered software released into the public domain.

use alloc::{boxed::Box, string::String, vec::Vec};
use core::error::Error;
use distrib_common::{BoxError, Build, Clean, PackageRegistry, Publish, Tool, Utf8PathBuf};
use std::process::Command;

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

impl<T> From<&T> for PipProgram
where
    T: Clone + Into<Self>,
{
    fn from(t: &T) -> Self {
        t.clone().into()
    }
}

impl From<PipProgram> for Command {
    fn from(input: PipProgram) -> Self {
        Command::new(input.0)
    }
}

impl PipProgram {}

impl Tool for PipProgram {}

impl Clean for PipProgram {
    fn clean(&self) -> Result<(), BoxError> {
        Ok(())
    }
}

impl Build for PipProgram {
    fn build(&self) -> Result<(), BoxError> {
        let mut cmd = Command::from(self.clone());
        cmd.args(["build"]);

        #[cfg(feature = "tracing")]
        tracing::info!("Executing {:?}", cmd);

        cmd.status().map_err(|err| Box::new(err))?;
        Ok(())
    }
}

impl Publish for PipProgram {
    fn publish(&self, _registry: Option<PackageRegistry>) -> Result<(), BoxError> {
        todo!() // TODO
    }
}
