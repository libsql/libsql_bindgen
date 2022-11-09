# libSQL user-defined functions: encrypt-decrypt

**_NOTE:_** This experimental example is based on top of pull request https://github.com/libsql/libsql/pull/45 - it's not in upstream libSQL at the time of writing this post!

This short example shows how to write simple encryption/decryption routines in Rust, compile them to WebAssembly and finally register them as user-defined functions in libSQL command-line interface.

## Initial setup
### Install cargo
Support for user-defined functions is currently implemented on top of [Wasmtime](https://github.com/bytecodealliance/wasmtime). Our roadmap includes evaluating [WasmEdge](https://github.com/WasmEdge/WasmEdge) and [Wasmer](https://github.com/wasmerio/wasmer) as well. 

In order to be able to build the runtime, follow the official instructions: https://doc.rust-lang.org/cargo/getting-started/installation.html

Once done, libSQL can be compiled with WebAssembly functions support enabled.

### Compile libSQL with support for WebAssembly functions
Support for WebAssembly-powered user-defined functions is currently experimental and opt-in. It can be enabled by compiling libSQL with the `--enable-wasm-runtime` configure flag. In order to compile libSQL shell with WebAssembly support, the following snippet can be used:

```sh
./configure --enable-wasm-runtime
make -j8 sqlite3
```

Remember that this configuration option is only present at https://github.com/libsql/libsql/pull/45 right now.

Once compiled, the shell binary will depend on `libwblibsql.so` library produced in `.libs/` directory.
In order to be able to run the shell, either move the binary to a known library path (e.g. `/usr/lib`),
or use the following env variable (Linux-only):
```sh
LD_LIBRARY_PATH=.libs ./sqlite3
```

# Compile user-defined functions to WebAssembly

## libsql\_bindgen macro
libSQL supports running WebAssembly functions with specific type translation: 
 - INTEGER and DOUBLE are passed as is
 - TEXT is passed as a pointer to the following structure:
    `[1 byte of type information][null-terminated string]`
 - BLOB is passed as a pointer to the following structure:
    `[1 byte of type information][4 bytes of big endian size][data]`
 - NULL is passed as a pointer to the following structure:
    `[1 byte of type information]`

In order to automatically translate between native Rust types and libSQL types,
one can use the [libsql\_bindgen](https://crates.io/crates/libsql_bindgen) crate
and its [#[libsql\_bindgen]](https://docs.rs/libsql_bindgen/latest/libsql_bindgen/attr.libsql_bindgen.html) macro. Here's an example:
https://github.com/psarna/libsql_bindgen/blob/3e215e270773d101440c8d0e93b730e2107a4dd3/examples/encrypt_decrypt/src/lib.rs#L4-L8

Full implementation of encryption/decryption is not much longer and can be found here:
https://github.com/psarna/libsql_bindgen/blob/3e215e270773d101440c8d0e93b730e2107a4dd3/examples/encrypt_decrypt/src/lib.rs#L1-L15

Even though the code operates on native Rust type - `String`, the generated WebAssembly output will correctly translate it to pointers to structures mentioned above.

Finally, please note that even though this code relies on an external crate - magic-crypt - it's still fully compilable to standalone WebAssembly, and can run directly in libSQL!

## get\_sql.sh script

A convenience script is available for producing an SQL snippet which creates given user-defined function. The script takes one parameter - the exported function name - and outputs a single file named `create_<function-name>.sql`, which can be run in libSQL shell to register the function.

This example contains the implementation of two functions - `encrypt` and `decrypt`. Their SQL can be generated as follows:
```sh
./get_sql.sh encrypt
./get_sql.sh decrypt
ls libsql-target/*.sql
```
On success, the following files should appear:
```sh
[sarna@sarna-pc encrypt_decrypt]$ ls -lsh libsql-target/*.sql
816K -rw-r--r-- 1 sarna sarna 813K Nov  2 11:21 libsql-target/create_decrypt.sql
816K -rw-r--r-- 1 sarna sarna 813K Nov  2 12:34 libsql-target/create_encrypt.sql
```

Feel free to inspect these files - it's a regular `CREATE FUNCTION` statement, but it *will* be quite large due to the size of Wasm binary.

> **_NOTE:_** The binary is large mostly due to containing lots of Rust standard library inside. We plan to significantly reduce the output size, either by aggresive optimization and dropping unused symbols, or by shipping the Rust runtime as a separate, deduplicated Wasm module.

## Use the functions!
Once the SQL commands are generated, they can be used to dynamically register the functions - either in libSQL shell, or via any other driver.

Here's an example shell snippet:
```sql
sqlite3 # remember that this binary needs to be compiled with Wasm support


.init_wasm_func_table -- sets up a table for storing functions, it's enough to invoke it once per session
.read libsql-target/create_encrypt.sql
.read libsql-target/create_decrypt.sql

CREATE TABLE secrets(secret);
INSERT INTO secrets (secret) VALUES (encrypt('my secret value: 1', 's3cretp4ss'));
INSERT INTO secrets (secret) VALUES (encrypt('my even more secret value: 2', 's3cretp4ss'));
INSERT INTO secrets (secret) VALUES (encrypt('classified value: 3', 's3cretp4ss'));

.mode column
SELECT secret, decrypt(secret, 'wrong-pass') from secrets;

secret                                        decrypt(secret, 'wrong-pass')
--------------------------------------------  -----------------------------
IyTvoTEnh9a/f6+pac3rLPToP9DkWqS7CEW8tan3mbQ=  [ACCESS DENIED]              
bUQ4fEe6hPnsMx8ABOZO97CMr/wouGTByfUCEmFVZTs=  [ACCESS DENIED]              
o+m1w7UdoxBZxLumNW0VoMKSMFaC4o8N5uknAQZ/yFY=  [ACCESS DENIED] 

SELECT secret, decrypt(secret, 's3cretp4ss') from secrets;
secret                                        decrypt(secret, 's3cretp4ss')
--------------------------------------------  -----------------------------
IyTvoTEnh9a/f6+pac3rLPToP9DkWqS7CEW8tan3mbQ=  my secret value: 1           
bUQ4fEe6hPnsMx8ABOZO97CMr/wouGTByfUCEmFVZTs=  my even more secret value: 2 
o+m1w7UdoxBZxLumNW0VoMKSMFaC4o8N5uknAQZ/yFY=  classified value: 3 

SELECT secret, decrypt(secret, 's3cretp4ss') from secrets;

```

