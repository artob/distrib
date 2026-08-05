// This is free and unencumbered software released into the public domain.

use alloc::{boxed::Box, string::String, vec::Vec};
use core::error::Error;
use distrib_common::{Build, Utf8PathBuf};
use std::process::Command;

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

impl<T> From<&T> for NpmProgram
where
    T: Clone + Into<Self>,
{
    fn from(t: &T) -> Self {
        t.clone().into()
    }
}

impl From<NpmProgram> for Command {
    fn from(input: NpmProgram) -> Self {
        Command::new(input.0)
    }
}

impl NpmProgram {}

impl Build for NpmProgram {
    fn build(&self) -> Result<(), Box<dyn Error>> {
        let mut cmd = Command::from(self.clone());
        cmd.args(["run", "build"]);

        #[cfg(feature = "tracing")]
        tracing::info!("Executing {:?}", cmd);

        cmd.status().map_err(|err| Box::new(err))?;
        Ok(())
    }
}
