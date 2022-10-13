# Intro

This repository currently contains a few random examples
of functions which compile to WebAssembly and can later
be used with https://github.com/libsql/libsql/
with WebAssembly integration enabled (https://github.com/libsql/libsql/issues/17).

### How to compile to WebAssembly

```sh
# only needed once, if you don't have this target already installed
rustup target add wasm32-unknown-unknown 

# remember about the release target, otherwise the generated code
# will have one billion lines of boilerplate
cargo build --release --target wasm32-unknown-unknown

wasm2wat target/wasm32-unknown-unknown/release/libsql_bindgen.wasm > output.wat
```

### How to inject it later into libSQL:
```sql
CREATE TABLE IF NOT EXISTS libsql_wasm_func_table(name text PRIMARY KEY, body text);

INSERT INTO libsql_wasm_func_table (name, body) VALUES ('fib', '
(module 
 (type (;0;) (func (param i64) (result i64))) 
 (func $fib (type 0) (param i64) (result i64) 
 (local i64) 
 i64.const 0 
 local.set 1 
 block ;; label = @1 
 local.get 0 
 i64.const 2 
 i64.lt_u 
 br_if 0 (;@1;) 
 i64.const 0 
 local.set 1 
 loop ;; label = @2 
 local.get 0 
 i64.const -1 
 i64.add 
 call $fib 
 local.get 1 
 i64.add 
 local.set 1 
 local.get 0 
 i64.const -2 
 i64.add 
 local.tee 0 
 i64.const 1 
 i64.gt_u 
 br_if 0 (;@2;) 
 end 
 end 
 local.get 0 
 local.get 1 
 i64.add) 
 (memory (;0;) 16) 
 (global $__stack_pointer (mut i32) (i32.const 1048576)) 
 (global (;1;) i32 (i32.const 1048576)) 
 (global (;2;) i32 (i32.const 1048576)) 
 (export "memory" (memory 0)) 
 (export "fib" (func $fib)))
');
```

Once done, you can use the function as if it was created with SQL's CREATE FUNCTION statement:
```sql
CREATE TABLE IF NOT EXISTS example(id int PRIMARY KEY);
INSERT OR REPLACE INTO example(id) VALUES (7);
INSERT OR REPLACE INTO example(id) VALUES (8);
INSERT OR REPLACE INTO example(id) VALUES (9);
SELECT id, fib(id) FROM example;
```
