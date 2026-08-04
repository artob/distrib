// This is free and unencumbered software released into the public domain.

use alloc::string::String;
use derive_more::{Display, FromStr};

#[derive(Clone, Debug, Display, Default, Eq, FromStr, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[non_exhaustive]
pub enum Runtime {
    /// The BEAM (aka Erlang/OTP) runtime.
    ///
    /// See: <https://www.erlang.org/blog/a-brief-beam-primer/>
    Beam,

    /// The Deno runtime.
    ///
    /// See: <https://deno.com/>
    #[cfg(feature = "unstable")]
    Deno,

    /// The .NET runtime.
    ///
    /// See: <https://dotnet.microsoft.com/>
    #[cfg(feature = "unstable")]
    Dotnet,

    /// The JVM (Java Virtual Machine) runtime.
    ///
    /// See: <https://java.com/>
    #[cfg(feature = "unstable")]
    Jvm,

    /// The Node.js runtime.
    ///
    /// See: <https://nodejs.org/>
    Node,

    /// The WebAssembly (aka Wasm) runtime.
    ///
    /// See: <https://webassembly.org/>
    #[default]
    WebAssembly,
}
