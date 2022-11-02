#!/usr/bin/env bash

set -e

if [ "$#" -ne 1 ]; then
    echo "Usage: $0 <function-name>"
    exit 1
fi

LIBSQL_EXPORTED_FUNC=$1
LIBSQL_COMPILED_WASM=libsql-target/wasm32-unknown-unknown/release/libsql_encrypt_decrypt.wasm
LIBSQL_OPTIMIZED_WASM=libsql-target/libsql_encrypt_decrypt_optimized.wasm
LIBSQL_TARGET_FILE=libsql-target/create_${LIBSQL_EXPORTED_FUNC}.sql

CARGO_TARGET_DIR=libsql-target cargo build --release --target wasm32-unknown-unknown
wasm-opt -Oz $LIBSQL_COMPILED_WASM -o $LIBSQL_OPTIMIZED_WASM || :
echo ".init_wasm_func_table -- only needed for shell" > $LIBSQL_TARGET_FILE
echo "DROP FUNCTION IF EXISTS $1;" >> $LIBSQL_TARGET_FILE
echo "CREATE FUNCTION $1 LANGUAGE wasm AS '" >> $LIBSQL_TARGET_FILE
wasm2wat $LIBSQL_OPTIMIZED_WASM | sed "s/'/''/g" >> $LIBSQL_TARGET_FILE
echo "';" >> $LIBSQL_TARGET_FILE

if ! grep "export \"$LIBSQL_EXPORTED_FUNC\"" $LIBSQL_TARGET_FILE; then
    echo "Error: function $LIBSQL_EXPORTED_FUNC not exported from the WebAssembly module"
    exit 1
fi
if ! grep "export \"memory\"" $LIBSQL_TARGET_FILE; then
    echo "Error: memory not exported from the WebAssembly module"
    exit 1
fi
