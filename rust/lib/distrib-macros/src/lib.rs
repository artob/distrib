// This is free and unencumbered software released into the public domain.

//! Distrib helps you distribute your software.

#![no_std]
#![allow(unused)]
#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

use alloc::string::String;
use darling::FromMeta;
use proc_macro::TokenStream;
use quote::quote;
use syn::ItemFn;

#[derive(Clone, Debug, Default, FromMeta)]
#[darling(default, derive_syn_parse)]
struct ExportArgs {
    rename: Option<ExportRenameArgs>,
}

#[derive(Clone, Debug, Default, FromMeta)]
#[darling(default, derive_syn_parse)]
struct ExportRenameArgs {
    beam: Option<String>,
    dart: Option<String>,
    js: Option<String>,
    python: Option<String>,
    ruby: Option<String>,
    rust: Option<String>,
    wasm: Option<String>,
}

/// A proc macro used to export Rust types to other languages.
#[proc_macro_attribute]
pub fn export(args: TokenStream, input: TokenStream) -> TokenStream {
    let args: ExportArgs = match syn::parse(args) {
        Ok(args) => args,
        Err(err) => {
            return err.to_compile_error().into();
        },
    };
    //std::dbg!(&args); // DEBUG

    let input = syn::parse_macro_input!(input as ItemFn);

    // See: <https://crates.io/crates/napi>
    // See: <https://crates.io/crates/napi-derive>
    // See: <https://docs.rs/napi-derive/latest/napi_derive/attr.napi.html>
    let napi = if cfg!(feature = "napi") {
        //let prefix = quote! { ::distrib::js::napi_derive };
        //let napi = quote! { #prefix::napi };
        //if let Some(name) = args.rename.as_ref().and_then(|r| r.js.clone()) {
        //    // See: https://github.com/napi-rs/napi-rs/blob/main/crates/macro/src/parser/attrs.rs#L50
        //    quote! { #[#napi(js_name = #name)] }
        //} else {
        //    quote! { #[#napi] }
        //}
        quote! {} // TODO: this requires a direct dependency on napi:
    } else {
        quote! {}
    };

    // See: <https://crates.io/crates/pyo3>
    // See: <https://pyo3.rs/main/function.html>
    let pyo3 = if cfg!(feature = "pyo3") {
        let prefix = quote! { ::distrib::python::pyo3 };
        let prelude = quote! { #prefix::prelude };
        let pyfunction = quote! { #prelude::pyfunction };
        if let Some(name) = args.rename.as_ref().and_then(|r| r.python.clone()) {
            // See: https://github.com/PyO3/pyo3/blob/main/pyo3-macros-backend/src/pyfunction.rs
            quote! { #[#pyfunction] #[pyo3(crate = "::distrib::python::pyo3", name = #name)] }
        } else {
            quote! { #[#pyfunction] #[pyo3(crate = "::distrib::python::pyo3")] }
        }
    } else {
        quote! {}
    };

    // See: <https://crates.io/crates/wasm-bindgen>
    // See: <https://docs.rs/wasm-bindgen/latest/wasm_bindgen/prelude/attr.wasm_bindgen.html>
    // See: <https://wasm-bindgen.github.io/wasm-bindgen/reference/attributes/on-rust-exports/index.html>
    // See: <https://github.com/wasm-bindgen/wasm-bindgen/blob/main/crates/macro/Cargo.toml>
    let wasm_bindgen = if cfg!(feature = "wasm-bindgen") {
        let prefix = quote! { ::distrib::wasm::wasm_bindgen };
        let prelude = quote! { #prefix::prelude };
        let wasm_bindgen = quote! { #prelude::wasm_bindgen };
        if let Some(name) = args.rename.as_ref().and_then(|r| r.wasm.clone()) {
            // See: https://wasm-bindgen.github.io/wasm-bindgen/reference/attributes/on-rust-exports/js_name.html
            quote! { #[#wasm_bindgen(wasm_bindgen = #prefix, js_name = #name)] }
        } else {
            quote! { #[#wasm_bindgen(wasm_bindgen = #prefix)] }
        }
    } else {
        quote! {}
    };

    let output = quote! {
        #napi
        #pyo3
        #wasm_bindgen
        #input
    };
    //std::dbg!(std::string::ToString::to_string(&output)); // DEBUG
    output.into()
}
