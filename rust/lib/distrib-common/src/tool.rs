// This is free and unencumbered software released into the public domain.

pub trait Tool: Clean + Build + Publish {}

mod build;
pub use build::*;

mod clean;
pub use clean::*;

mod publish;
pub use publish::*;
