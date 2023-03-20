## How to use:

[Install WasmEdge](https://wasmedge.org/book/en/quick_start/install.html) and then install the HTTPS plugin as follows.

```bash
# Download and extract the plugin
wget "https://github.com/second-state/wasmedge_rustls_plugin/releases/download/0.1.0/WasmEdge-plugin-wasmedge_rustls-0.1.0-alpha-ubuntu20.04_x86_64.tar"

tar -xf "WasmEdge-plugin-wasmedge_rustls-0.1.0-alpha-ubuntu20.04_x86_64.tar"

# Install the plugin if your wasmedge is installed in ~/.wasmedge
cp libwasmedge_rustls.so ~/.wasmedge/plugin/

# Install the plugin if your wasmedge is installed in /usr/local
cp libwasmedge_rustls.so /usr/local/lib/wasmedge/
```

Build the [GET EXCHANGE RATE](src/main.rs) example.

```bash
cargo wasi build --release
```

Create sql file for libsql and run libsql
```bash
./gen_libsql_udf.sh

libsql
```

Execute in libsql
```sql
> .init_wasm_func_table
> .read create_get_exchange_rate_udf.sql
> select get_exchange_rate('USD','CNY');
```
