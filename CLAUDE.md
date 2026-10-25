# English, rewritten in Rust

A Cargo workspace with no external dependencies.

- `crates/english`: English (Rust edition). The `english!` macro (in `lib.rs`) type-checks sentences; `grammar.rs` holds the `Takes*` agreement traits and their `on_unimplemented` messages; `lexicon.rs` holds the irregular tables and the `const fn` classifiers the macro calls at compile time.
- `crates/englishc`: the prose compiler. `lexer.rs` builds tokens and sentences, `lints/` holds one module per family of mistakes, `render.rs` reproduces rustc's layout (plain, ANSI or HTML), `web.rs` is the wasm C ABI used by the playground.
- `playground/`: `index.html` plus `examples/*.rs`; `build.py` writes `playground/dist/` (gitignored).

## Commands

- `cargo test` runs unit tests, doctests and the `compile_fail` grammar examples.
- `cargo run -p englishc -- examples/essay.txt` shows every diagnostic; add `--fix` to apply them.
- `python3 -I playground/build.py` rebuilds the wasm and re-records real rustc output for the crate examples.

## Conventions

- `englishc` and the crate share `english::lexicon`. Add irregular forms there, not in a lint.
- A lint fires only where a neighbouring word rules out the correct reading. Every new rule needs a positive and a negative case in `crates/englishc/src/lib.rs` tests.
- Before committing a lint change, run the compiler over a large body of edited prose (the Rust book's HTML under `~/.rustup/toolchains/*/share/doc/rust/html/book/` works, with code blocks stripped) and check that errors stay at zero. The last run, on about 150,000 words, gave 0 errors and 17 warnings.
- Playground outputs must be real: `build.py` records rustc output by compiling the examples. Don't hand-edit them.
- The published playground is the It page `english-in-rust`. After rebuilding, republish with `it update english-in-rust --dir playground/dist`.
