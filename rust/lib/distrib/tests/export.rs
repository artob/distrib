// This is free and unencumbered software released into the public domain.

#![cfg(feature = "export")]

#[cfg(feature = "python")]
#[distrib::export(rename(python = "renamed_square", wasm = "renamedSquare"))]
pub fn square(value: i32) -> i32 {
    value * value
}

#[cfg(feature = "python")]
#[distrib::export]
pub fn add(left: i32, right: i32) -> i32 {
    left + right
}

#[cfg(feature = "python")]
#[test]
fn python_export_is_callable_under_its_renamed_name() {
    use distrib::python::pyo3::{self, prelude::*};

    Python::initialize();
    Python::attach(|py| {
        let function = pyo3::wrap_pyfunction!(square, py).expect("exported Python function");
        let name: String = function.getattr("__name__").unwrap().extract().unwrap();
        assert_eq!(name, "renamed_square");
        let result: i32 = function.call1((7,)).unwrap().extract().unwrap();
        assert_eq!(result, 49);

        let function = pyo3::wrap_pyfunction!(add, py).expect("default Python function name");
        let name: String = function.getattr("__name__").unwrap().extract().unwrap();
        assert_eq!(name, "add");
        let result: i32 = function.call1((2, 3)).unwrap().extract().unwrap();
        assert_eq!(result, 5);
    });
}

#[cfg(not(any(feature = "python", feature = "wasm")))]
#[distrib::export]
fn identity<T>(value: T) -> T {
    value
}

#[cfg(not(any(feature = "python", feature = "wasm")))]
#[test]
fn export_without_a_backend_preserves_rust_functions() {
    assert_eq!(identity("Rust only"), "Rust only");
}
