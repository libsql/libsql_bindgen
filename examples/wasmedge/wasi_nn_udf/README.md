## How to use:

[Install WasmEdge](https://wasmedge.org/book/en/quick_start/install.html) and then install the HTTPS plugin as follows.

```bash
# Download and extract the plugin
wget "https://github.com/WasmEdge/WasmEdge/releases/download/0.12.0-alpha.2/WasmEdge-plugin-wasi_nn-pytorch-0.12.0-alpha.2-ubuntu20.04_x86_64.tar.gz"

tar -xzf "WasmEdge-plugin-wasi_nn-pytorch-0.12.0-alpha.2-ubuntu20.04_x86_64.tar.gz"

# Install the plugin if your wasmedge is installed in ~/.wasmedge
cp libwasmedgePluginWasiNN.so ~/.wasmedge/plugin/

# Install the plugin if your wasmedge is installed in /usr/local
cp libwasmedgePluginWasiNN.so /usr/local/lib/wasmedge/
```

Build the [WASI NN UDF](src/main.rs) example.

```bash
cargo wasi build --release
```

Create test.sql file for libsql and run libsql
```bash
./gen_demo_sql.sh

libsql
```

Execute in libsql
```sql
> .read test.sql
```
