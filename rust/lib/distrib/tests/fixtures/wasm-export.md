A supported signature can be renamed and remains callable from Rust:

```rust
#[distrib::export(rename(wasm = "renamedSquare"))]
pub fn square(value: u32) -> u32 {
    value * value
}
assert_eq!(square(7), 49);
```

The backend must reject a generic function that cannot cross the Wasm boundary.
If export-feature forwarding regresses to a no-op, this compile-fail test fails:

```compile_fail
#[distrib::export]
pub fn identity<T>(value: T) -> T {
    value
}
```
