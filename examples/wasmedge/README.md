# WasmEdge examples

[WasmEdge](https://github.com/WasmEdge/WasmEdge) is a high-performance WebAssembly (Wasm) runtime [optimized for cloud-native applications](https://wasmedge.org/docs/develop/wasmedge/features). When [embedded in libsql](https://wasmedge.org/docs/embed/use-case/libsql#prerequisites), it can support complex Wasm UDF functions with advanced features. In this folder, we will give two examples that leverage WasmEdge-specific features.

The [Get exchange rate](get_exchange_rate/) example shows how a libsql UDF can make external web service calls. In this case, it makes an HTTPS request to get the latest exchange rate between any two pair of currencies. You can then use the exchange rate in your queries.

The [WASI NN](wasi_nn_udf/) example shows how a libsql UDF can read a binary image stored in a blob fields, and then use a PyTorch model, stored inside the UDF, to classify the content in the image. The UDF outputs the text label of the image classification.
