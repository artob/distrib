// This is free and unencumbered software released into the public domain.

use alloc::{boxed::Box, string::String, vec::Vec};
use core::error::Error;
use distrib_common::{Build, Utf8PathBuf};
use std::{ffi::OsString, process::Command};

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

impl<T> From<&T> for DartProgram
where
    T: Clone + Into<Self>,
{
    fn from(t: &T) -> Self {
        t.clone().into()
    }
}

impl From<DartProgram> for Command {
    fn from(input: DartProgram) -> Self {
        Command::new(input.0)
    }
}

impl DartProgram {}

impl Build for DartProgram {
    fn build(&self) -> Result<(), Box<dyn Error>> {
        let mut cmd = Command::from(self.clone());
        cmd.args(["build", "cli"]);

        #[cfg(feature = "tracing")]
        tracing::info!("Executing {:?}", cmd);

        cmd.status().map_err(|err| Box::new(err))?;
        Ok(())
    }
}
