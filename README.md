# Luhney

Implementation of Luhns Algorithm, exposed as CLI, REPL, and WASM chatbot-like REPL.

## REPL

`cargo r --bin repl --features="repl-deps"`

## CLI

`cargo r --bin cli --features="cli-deps" -- --input test_ccs.txt`

## WASM

`wasm-pack build --target web`

## Nothing, just a luhn algorithm library

By default, the WASM bindings are generated to be easier to use with `wasm-pack`.
To build without generating and including the bindings:

`cargo b --no-default-features` or `default-features = false` in your `Cargo.toml`
(but really this library is intended as a demo/learning project, check before you use in production)