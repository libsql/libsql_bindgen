#!/bin/sh
FUNC_NAME='classify'
echo "DROP FUNCTION IF EXISTS ${FUNC_NAME};" > create_${FUNC_NAME}_udf.sql
echo -n "CREATE FUNCTION ${FUNC_NAME} LANGUAGE wasm AS X'" >> create_${FUNC_NAME}_udf.sql
xxd -p $1 | tr -d "\n" >> create_${FUNC_NAME}_udf.sql 
echo "';" >> create_${FUNC_NAME}_udf.sql