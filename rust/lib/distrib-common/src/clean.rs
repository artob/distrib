// This is free and unencumbered software released into the public domain.

use alloc::boxed::Box;
use core::error::Error;

pub trait Clean {
    fn clean(&self) -> Result<(), Box<dyn Error>>;
}
