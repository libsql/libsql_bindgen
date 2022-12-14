# Intro

This repository contains a helper library for coding
WebAssembly-powered user-defined functions for libSQL.

Marking a native Rust function with `#[libsql_bindgen]` macro
and compiling it to `wasm32-unknown-unknown` target
is enough to produce a user-defined function definition
callable directly from libSQL.

Example:
https://github.com/psarna/libsql_bindgen/blob/master/examples/encrypt_decrypt/src/lib.rs

Try it yourself:
```
cd examples/encrypt_decrypt
./get_sql.sh encrypt
./get_sql.sh decrypt
```

This repository is the foundation of [libSQL generate](https://github.com/psarna/libsql_generate) and http://bindgen.libsql.org/
