// This is free and unencumbered software released into the public domain.

use alloc::{boxed::Box, string::String, vec::Vec};
use core::error::Error;
use distrib_common::{Build, Utf8PathBuf};
use std::process::Command;

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

impl<T> From<&T> for GleamProgram
where
    T: Clone + Into<Self>,
{
    fn from(t: &T) -> Self {
        t.clone().into()
    }
}

impl From<GleamProgram> for Command {
    fn from(input: GleamProgram) -> Self {
        Command::new(input.0)
    }
}

impl GleamProgram {}

impl Build for GleamProgram {
    fn build(&self) -> Result<(), Box<dyn Error>> {
        let mut cmd = Command::from(self.clone());
        cmd.args(["build"]);

        #[cfg(feature = "tracing")]
        tracing::info!("Executing {:?}", cmd);

        cmd.status().map_err(|err| Box::new(err))?;
        Ok(())
    }
}
