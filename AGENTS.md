# Distrib agent guide

Distrib is a pre-alpha software-distribution CLI and polyglot library. Work only
inside this repository; do not inspect parent directories.

## Code map

- `rust/`: Cargo workspace, members `lib/*`; shared dependencies/local patches
  in `Cargo.toml`. Edition 2024; declared minimum Rust 1.85.
- `rust/lib/distrib/src/`: `main.rs` handles CLI/discovery/dispatch; `config.rs`
  models `.config/distrib.toml`; `package.rs` normalizes metadata;
  `package/*.rs` converts ecosystem manifests to/from `Package`.
- `rust/lib/distrib-common/`: enums and `Tool`/`Build`/`Clean`/`Publish` traits.
  Ecosystem crates own manifest models, `load.rs`, and `program.rs` adapters;
  `distrib-macros` owns `#[export]`; `distrib-build` owns build-script helpers.
- `beam/`: Gleam CLI wrapper. `elixir`, `erlang`, `gleam` are symlinks to it.
- `ruby/lib/distrib/ruby/`: gemspec and Rake helpers. `dart/`, `js/`, `python/`
  are mostly package scaffolding; Python distribution name is `distrib.sh`.

## Implementation rules

- Trace behavior through loader → `Package` conversion → CLI/tool dispatch.
  TODOs, enum variants, and empty implementations do not prove working support.
- Preserve `#![forbid(unsafe_code)]` and the `no_std`/`alloc`/`std` feature split.
  Gate optional imports/APIs with their dependencies.
- Return contextual errors for bad manifests and failed subprocesses; do not
  copy existing input-dependent `unwrap()`/`todo!()` or ignored exit statuses.
- For behavior changes, add regression tests with fixture manifests/fake
  executables. Never test by publishing real packages.
- Aim for rustdoc on every public symbol, including errors, feature requirements,
  and useful examples. Detailed documentation belongs beside modules/types;
  README additions require clear user-facing value.

## Validation

Run relevant checks **from `rust/`**; root `make`/`rake` generate documentation:

```sh
cargo fmt --all -- --check
cargo test --workspace --locked
cargo test --workspace --locked --features distrib/cli
cargo clippy --workspace --all-targets --locked --features distrib/cli
cargo doc --workspace --no-deps --locked --features distrib/cli
cargo run -p distrib --locked --features cli -- --help
```

CLI requires `cli`; defaults omit it. Check changed crates independently with
`cargo check -p NAME --locked`; feature changes also need `--no-default-features`
and intended feature combinations. Workspace feature unification can mask bugs.
`--all-features` pulls native FFI dependencies; it is not the baseline.
Known gaps: no-default-feature builds fail; locked dependencies exceed Rust 1.85.
Report actual failures; compilation alone is not behavioral coverage.

Other packages (commands run in the named directory):
- `beam/`: `gleam test`; `gleam format --check src test`.
- `js/`: use Bun; `bun run build`; `bunx --no-install tsc --noEmit`.
- `dart/`: `dart pub get`, then `dart analyze` and `dart test`.
- Python/Ruby: smoke-test affected APIs; no substantive test suites yet.

## Generated files and releases

- Root and Dart/JS/Python/Ruby/Rust READMEs: edit `.config/readmer/` templates,
  then regenerate the corresponding Make target.
- Release workflow: edit `dist-workspace.toml`, then regenerate
  `.github/workflows/release.yml` using the pinned cargo-dist version.
- Regenerate artifacts/lockfiles with their owning tools; never hand-edit them.
- For version changes, reconcile `VERSION` files, ecosystem manifests, exact
  Rust dependency pins, and changelogs; existing values differ.
