## How to use:

[Install WasmEdge](https://wasmedge.org/book/en/quick_start/install.html) and then install the WasiNN plugin as follows.

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

Make sure that you [build libsql with WasmEdge support](https://wasmedge.org/docs/embed/use-case/libsql#prerequisites).

```bash
git clone https://github.com/libsql/libsql
cd libsql
./configure --enable-wasm-runtime-wasmedge
make
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

### Note

The [gen_demo_sql.sh](gen_demo_sql.sh) script 

* first converts the image file to a tensor file. This is not strictly necessary as the UDF itself can perform this conversion. 
* then calls [gen_insert_image_sql.sh](gen_insert_image_sql.sh) to create a SQL file that inserts the tensor file into a database table as a blob.
* then creates a database table with the above mentioned blob field, and calls the generated SQL file to insert the blob.
* then calls [gen_libsql_udf.sh](gen_libsql_udf.sh) to create the UDF. The PyTorch model is embedded in the UDF.
* finally, uses the UDF to classify the blob in a SQL query.
