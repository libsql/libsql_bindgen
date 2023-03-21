#!/bin/sh
FUNC_NAME='get_exchange_rate'
echo "DROP FUNCTION IF EXISTS ${FUNC_NAME};" > create_${FUNC_NAME}_udf.sql
echo -n "CREATE FUNCTION ${FUNC_NAME} LANGUAGE wasm AS X'" >> create_${FUNC_NAME}_udf.sql
xxd -p ../../../target/wasm32-wasi/release/get_exchange_rate.wasm | tr -d "\n" >> create_${FUNC_NAME}_udf.sql 
echo "';" >> create_${FUNC_NAME}_udf.sql