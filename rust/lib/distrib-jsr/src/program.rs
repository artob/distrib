// This is free and unencumbered software released into the public domain.

use alloc::boxed::Box;
use core::error::Error;
use distrib_common::{Build, Utf8PathBuf};
use std::process::Command;

pub const JSR_COMMAND: &str = "npx jsr";

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct JsrProgram(Utf8PathBuf);

impl Default for JsrProgram {
    fn default() -> Self {
        Self(JSR_COMMAND.into())
    }
}

impl From<Utf8PathBuf> for JsrProgram {
    fn from(input: Utf8PathBuf) -> Self {
        Self(input)
    }
}

impl<T> From<&T> for JsrProgram
where
    T: Clone + Into<Self>,
{
    fn from(t: &T) -> Self {
        t.clone().into()
    }
}

impl From<JsrProgram> for Command {
    fn from(input: JsrProgram) -> Self {
        Command::new(input.0)
    }
}

impl JsrProgram {}

impl Build for JsrProgram {
    fn build(&self) -> Result<(), Box<dyn Error>> {
        Command::from(self.clone())
            .args(["run", "build"])
            .status()
            .map_err(|err| Box::new(err))?;
        Ok(())
    }
}
