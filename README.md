# Intro

This repository contains a helper library for coding
WebAssembly-powered user-defined functions for libSQL.

Marking a native Rust function with `#[libsql_bindgen]` macro
and compiling it to `wasm32-unknown-unknown` target
is enough to produce a user-defined function definition
callable directly from libSQL.

TODO:
 * document, provide examples
 * generate a snippet script (or build.rs) for injecting a new function into libSQL, in form of an SQL query

