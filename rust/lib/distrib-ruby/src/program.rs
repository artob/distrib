// This is free and unencumbered software released into the public domain.

use alloc::{boxed::Box, string::String, vec::Vec};
use core::error::Error;
use distrib_common::{Build, Clean, Tool, Utf8PathBuf};
use std::process::Command;

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

impl<T> From<&T> for GemProgram
where
    T: Clone + Into<Self>,
{
    fn from(t: &T) -> Self {
        t.clone().into()
    }
}

impl From<GemProgram> for Command {
    fn from(input: GemProgram) -> Self {
        Command::new(input.0)
    }
}

impl GemProgram {}

impl Tool for GemProgram {}

impl Clean for GemProgram {
    fn clean(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

impl Build for GemProgram {
    fn build(&self) -> Result<(), Box<dyn Error>> {
        let mut cmd = Command::from(self.clone());
        cmd.args(["build"]);

        #[cfg(feature = "tracing")]
        tracing::info!("Executing {:?}", cmd);

        cmd.status().map_err(|err| Box::new(err))?;
        Ok(())
    }
}
