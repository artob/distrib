// This is free and unencumbered software released into the public domain.

use alloc::{boxed::Box, string::String, vec::Vec};
use core::error::Error;
use distrib_common::{Build, Clean, PackageRegistry, Publish, Tool, Utf8PathBuf};
use std::process::Command;

pub const JSR_COMMAND: &str = "jsr";

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

impl Tool for JsrProgram {}

impl Clean for JsrProgram {
    fn clean(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

impl Build for JsrProgram {
    fn build(&self) -> Result<(), Box<dyn Error>> {
        let mut cmd = Command::new("npx");
        cmd.args(["jsr", "run", "build"]);

        #[cfg(feature = "tracing")]
        tracing::info!("Executing {:?}", cmd);

        cmd.status().map_err(|err| Box::new(err))?;
        Ok(())
    }
}

impl Publish for JsrProgram {
    fn publish(&self, _registry: Option<PackageRegistry>) -> Result<(), Box<dyn Error>> {
        todo!() // TODO
    }
}
