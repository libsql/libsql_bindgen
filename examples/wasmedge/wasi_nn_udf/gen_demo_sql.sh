#!/bin/sh

# AOT wasm
wasmedgec ../../../target/wasm32-wasi/release/wasi_nn_udf.wasm wasi_nn_udf.aot.wasm

# create libsql udf
./gen_libsql_udf.sh ../../../target/wasm32-wasi/release/wasi_nn_udf.wasm

# image to tensor
wasmedge --dir=.:. wasi_nn_udf.aot.wasm ./input.jpg ./input.tensor

# create insert sql script
./gen_insert_image_sql.sh images img_blob input.tensor insert_data.sql

# create test sql script
SQL_NAME=${1:-"test.sql"}
echo ".init_wasm_func_table" > $SQL_NAME
echo ".read $PWD/create_classify_udf.sql" >> $SQL_NAME
echo "CREATE TABLE images (id INTEGER PRIMARY KEY AUTOINCREMENT,img_blob BLOB);" >> $SQL_NAME
echo ".read $PWD/insert_data.sql" >> $SQL_NAME
echo "select classify(img_blob) from images where id = 1;" >> $SQL_NAME