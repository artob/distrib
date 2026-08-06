// This is free and unencumbered software released into the public domain.

use alloc::string::String;
use derive_more::{Display, FromStr};

/// A supported programming language.
///
/// See: <https://www.tiobe.com/tiobe-index/>
#[derive(Clone, Debug, Display, Default, Eq, FromStr, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(
    feature = "serde",
    serde(rename_all(serialize = "lowercase", deserialize = "lowercase"))
)]
#[non_exhaustive]
pub enum Language {
    /// The Ada language.
    ///
    /// See: <https://ada-lang.io/>
    #[cfg(feature = "unstable")]
    Ada,

    /// The C language.
    ///
    /// See: <https://en.cppreference.com/w/c>
    #[cfg(feature = "unstable")]
    C,

    /// The Clojure language.
    ///
    /// See: <https://clojure.org/>
    #[cfg(feature = "unstable")]
    Clojure,

    /// The C++ language.
    ///
    /// See: <https://cppreference.com/>
    #[cfg(feature = "unstable")]
    Cpp,

    /// The Crystal language.
    ///
    /// See: <https://crystal-lang.org/>
    #[cfg(feature = "unstable")]
    Crystal,

    /// The C# language.
    ///
    /// See: <https://csharp.net/>
    #[cfg(feature = "unstable")]
    CSharp,

    /// The D language.
    ///
    /// See: <https://dlang.org/>
    #[cfg(feature = "unstable")]
    D,

    /// The Dart language.
    ///
    /// See: <https://dart.dev/>
    Dart,

    /// The Elixir language.
    ///
    /// See: <https://elixir-lang.org/>
    Elixir,

    /// The Erlang language.
    ///
    /// See: <https://erlang.org/>
    Erlang,

    /// The Fortran language.
    ///
    /// See: <https://fortran-lang.org/>
    #[cfg(feature = "unstable")]
    Fortran,

    /// The F# language.
    ///
    /// See: <https://fsharp.org/>
    #[cfg(feature = "unstable")]
    FSharp,

    /// The Gleam language.
    ///
    /// See: <https://gleam.run/>
    Gleam,

    /// The Go language.
    ///
    /// See: <https://go.dev/>
    #[cfg(feature = "unstable")]
    Go,

    /// The Haskell language.
    ///
    /// See: <https://haskell.org/>
    #[cfg(feature = "unstable")]
    Haskell,

    /// The Java language.
    ///
    /// See: <https://java.com/>
    #[cfg(feature = "unstable")]
    Java,

    /// The JavaScript (aka JS) language.
    ///
    /// See: <https://developer.mozilla.org/en-US/docs/Web/JavaScript>
    JavaScript,

    /// The Julia language.
    ///
    /// See: <https://julialang.org/>
    #[cfg(feature = "unstable")]
    Julia,

    /// The Kotlin language.
    ///
    /// See: <https://kotlinlang.org/>
    #[cfg(feature = "unstable")]
    Kotlin,

    /// The Common Lisp language.
    ///
    /// See: <https://common-lisp.net/>
    #[cfg(feature = "unstable")]
    Lisp,

    /// The Lua language.
    ///
    /// See: <https://lua.org/>
    #[cfg(feature = "unstable")]
    Lua,

    /// The Mojo language.
    ///
    /// See: <https://mojolang.org/>
    #[cfg(feature = "unstable")]
    Mojo,

    /// The Nim language.
    ///
    /// See: <https://nim-lang.org/>
    #[cfg(feature = "unstable")]
    Nim,

    /// The OCaml language.
    ///
    /// See: <https://ocaml.org/>
    #[cfg(feature = "unstable")]
    Ocaml,

    /// The Perl language.
    ///
    /// See: <https://perl.org/>
    #[cfg(feature = "unstable")]
    Perl,

    /// The PHP language.
    ///
    /// See: <https://php.net/>
    #[cfg(feature = "unstable")]
    Php,

    /// The Python language.
    ///
    /// See: <https://python.org/>
    Python,

    /// The R language.
    ///
    /// See: <https://r-project.org/>
    #[cfg(feature = "unstable")]
    R,

    /// The Racket language.
    ///
    /// See: <https://racket-lang.org/>
    #[cfg(feature = "unstable")]
    Racket,

    /// The Rhombus language.
    ///
    /// See: <https://rhombus-lang.org/>
    #[cfg(feature = "unstable")]
    Rhombus,

    /// The Ruby language.
    ///
    /// See: <https://ruby-lang.org/>
    Ruby,

    /// The Rust language.
    ///
    /// See: <https://rust-lang.org/>
    #[default]
    Rust,

    /// The Scala language.
    ///
    /// See: <https://scala-lang.org/>
    #[cfg(feature = "unstable")]
    Scala,

    /// The Swift language.
    ///
    /// See: <https://swift.org/>
    #[cfg(feature = "unstable")]
    Swift,

    /// The TypeScript (aka TS) language.
    ///
    /// See: <https://typescriptlang.org/>
    TypeScript,

    /// The Zig language.
    ///
    /// See: <https://ziglang.org/>
    #[cfg(feature = "unstable")]
    Zig,
}
