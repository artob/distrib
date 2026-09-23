// This is free and unencumbered software released into the public domain.

#[cfg(all(feature = "wasm", feature = "export"))]
#[distrib::export(rename(wasm = "renamedSquare"))]
pub fn square(value: u32) -> u32 {
    value * value
}

fn main() {}
