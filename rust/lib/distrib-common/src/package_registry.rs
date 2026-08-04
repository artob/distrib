// This is free and unencumbered software released into the public domain.

use alloc::string::String;
use derive_more::{Display, FromStr};

#[derive(Clone, Debug, Display, Eq, FromStr, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[non_exhaustive]
pub enum PackageRegistry {
    /// The Alire package registry.
    ///
    /// See: <https://alire.ada.dev/>
    Alire,

    /// The Clojars package registry for Clojure.
    ///
    /// See: <https://clojars.org/>
    Clojars,

    /// The CPAN package registry for Perl.
    ///
    /// See: <https://cpan.org/>
    Cpan,

    /// The Crates.io package registry.
    ///
    /// See: <https://crates.io/>
    Crates,

    /// The CRAN package registry for R.
    ///
    /// See: <https://cran.r-project.org/>
    Cran,

    /// The Hex.pm package registry.
    ///
    /// See: <https://hex.pm/>
    Hex,

    /// The JSR.io package registry.
    ///
    /// See: <https://jsr.io/>
    Jsr,

    /// The LuaRocks.org package registry.
    ///
    /// See: <https://luarocks.org/>
    LuaRocks,

    /// The NPM package registry.
    ///
    /// See: <https://npmjs.com/>
    Npm,

    /// The Pub.dev package registry.
    ///
    /// See: <https://pub.dev/>
    Pub,

    /// The PyPI.org package registry.
    ///
    /// See: <https://pypi.org/>
    PyPi,

    /// The RubyGems.org package registry.
    ///
    /// See: <https://rubygems.org/>
    RubyGems,
}
