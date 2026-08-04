// This is free and unencumbered software released into the public domain.

use alloc::string::String;
use derive_more::{Display, FromStr};

/// A supported package manager.
#[derive(Clone, Debug, Display, Eq, FromStr, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[non_exhaustive]
pub enum PackageManager {
    /// The Alire package manager for Ada.
    ///
    /// See: <https://alire.ada.dev/>
    #[cfg(feature = "unstable")]
    Alire,

    /// The Cabal package manager for Haskell.
    ///
    /// See: <https://haskell.org/cabal/>
    #[cfg(feature = "unstable")]
    Cabal,

    /// The Crates.io package manager for Rust.
    ///
    /// See: <https://crates.io/>
    Cargo,

    /// The Composer package manager for PHP.
    ///
    /// See: <https://getcomposer.org/>
    #[cfg(feature = "unstable")]
    Composer,

    /// The Conan package manager for C/C++.
    ///
    /// See: <https://conan.io/>
    #[cfg(feature = "unstable")]
    Conan,

    /// The CPAN package manager for Perl.
    ///
    /// See: <https://cpan.org/>
    #[cfg(feature = "unstable")]
    Cpan,

    /// The Gradle package manager for Java/JVM.
    ///
    /// See: <https://gradle.org/>
    #[cfg(feature = "unstable")]
    Gradle,

    /// The JSR.io package manager for TypeScript and ECMAScript.
    ///
    /// See: <https://jsr.io/>
    Jsr,

    /// The Leiningen package manager for Clojure.
    ///
    /// See: <https://leiningen.org/>
    #[cfg(feature = "unstable")]
    Lein,

    /// The LuaRocks.org package manager for Lua.
    ///
    /// See: <https://luarocks.org/>
    #[cfg(feature = "unstable")]
    LuaRocks,

    /// The Lux package manager for Lua.
    ///
    /// See: <https://lux.lumen-labs.org/>
    #[cfg(feature = "unstable")]
    Lux,

    /// The Maven package manager for Java/JVM.
    ///
    /// See: <https://maven.apache.org/>
    #[cfg(feature = "unstable")]
    Maven,

    /// The Hex.pm package manager for Elixir, Erlang, and Gleam.
    ///
    /// See: <https://hex.pm/>
    Mix,

    /// The Nimble package manager for Nim.
    ///
    /// See: <https://nimble.directory/>
    #[cfg(feature = "unstable")]
    Nimble,

    /// The NPM package manager for JavaScript and TypeScript.
    ///
    /// See: <https://npmjs.com/>
    Npm,

    /// The NuGet package manager for C#/.NET.
    ///
    /// See: <https://nuget.org/>
    #[cfg(feature = "unstable")]
    NuGet,

    /// The Opam package manager for OCaml.
    ///
    /// See: <https://opam.ocaml.org/>
    #[cfg(feature = "unstable")]
    Opam,

    /// The Pub.dev package manager for Dart.
    ///
    /// See: <https://pub.dev/>
    Pub,

    /// The PyPI.org package manager for Python.
    ///
    /// See: <https://pypi.org/>
    PyPi,

    /// The Raco package manager for Racket.
    ///
    /// See: <https://docs.racket-lang.org/raco/>
    #[cfg(feature = "unstable")]
    Raco,

    /// The RubyGems.org package manager for Ruby.
    ///
    /// See: <https://rubygems.org/>
    RubyGems,

    /// The Shards package manager for Crystal.
    ///
    /// See: <https://crystal-lang.org/reference/shards/>
    #[cfg(feature = "unstable")]
    Shards,

    /// The Swift Package Manager (SwiftPM) for Swift.
    ///
    /// See: <https://swift.org/package-manager/>
    #[cfg(feature = "unstable")]
    SwiftPm,

    /// The vcpkg package manager for C/C++.
    ///
    /// See: <https://vcpkg.io/>
    #[cfg(feature = "unstable")]
    Vcpkg,
}

impl PackageManager {
    pub fn program(&self) -> Option<&[&str]> {
        use PackageManager::*;
        Some(match self {
            Cargo => &["cargo"],
            Jsr => &["jsr", "npx jsr"],
            Mix => &["mix"],
            Npm => &["npm"],
            Pub => &["dart"],
            PyPi => &["uv"],
            #[cfg(feature = "unstable")]
            Raco => &["raco"],
            RubyGems => &["gem", "bundle exec gem"],
            _ => return None,
        })
    }
}
