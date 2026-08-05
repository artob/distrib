// This is free and unencumbered software released into the public domain.

use alloc::{boxed::Box, string::String, vec::Vec};
use core::error::Error;
use distrib_common::{Build, Clean, Tool, Utf8PathBuf};
use std::process::Command;

pub const CARGO_COMMAND: &str = "cargo";

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct CargoProgram(Utf8PathBuf);

impl Default for CargoProgram {
    fn default() -> Self {
        Self(CARGO_COMMAND.into())
    }
}

impl From<Utf8PathBuf> for CargoProgram {
    fn from(input: Utf8PathBuf) -> Self {
        Self(input)
    }
}

impl<T> From<&T> for CargoProgram
where
    T: Clone + Into<Self>,
{
    fn from(t: &T) -> Self {
        t.clone().into()
    }
}

impl From<CargoProgram> for Command {
    fn from(input: CargoProgram) -> Self {
        Command::new(input.0)
    }
}

impl CargoProgram {}

impl Tool for CargoProgram {}

impl Clean for CargoProgram {
    fn clean(&self) -> Result<(), Box<dyn Error>> {
        let mut cmd = Command::from(self.clone());
        cmd.args(["clean"]);

        #[cfg(feature = "tracing")]
        tracing::info!("Executing {:?}", cmd);

        cmd.status().map_err(|err| Box::new(err))?;
        Ok(())
    }
}

impl Build for CargoProgram {
    fn build(&self) -> Result<(), Box<dyn Error>> {
        let mut cmd = Command::from(self.clone());
        cmd.args(["build"]);

        #[cfg(feature = "tracing")]
        tracing::info!("Executing {:?}", cmd);

        cmd.status().map_err(|err| Box::new(err))?;
        Ok(())
    }
}
