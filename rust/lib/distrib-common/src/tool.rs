// This is free and unencumbered software released into the public domain.

use super::{Build, Clean};

pub trait Tool: Clean + Build {}
