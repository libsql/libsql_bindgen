## How to use:

[Install WasmEdge](https://wasmedge.org/book/en/quick_start/install.html) and then install the HTTPS plugin as follows.

```bash
# Install PyTorch
PYTORCH_VERSION="1.8.2"
curl -s -L -O --remote-name-all "https://download.pytorch.org/libtorch/lts/1.8/cpu/libtorch-cxx11-abi-shared-with-deps-${PYTORCH_VERSION}%2Bcpu.zip"
unzip -q "libtorch-cxx11-abi-shared-with-deps-${PYTORCH_VERSION}%2Bcpu.zip"
rm -f "libtorch-cxx11-abi-shared-with-deps-${PYTORCH_VERSION}%2Bcpu.zip"
export LD_LIBRARY_PATH=$(pwd)/libtorch/lib:${LD_LIBRARY_PATH}
export Torch_DIR=$(pwd)/libtorch

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
