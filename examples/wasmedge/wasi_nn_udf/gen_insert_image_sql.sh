#!/bin/sh
# example: ./gen_insert_image_sql.sh images img_blob input.jpg insert_data.sql

echo -n "INSERT INTO $1 ($2) VALUES(X'" >> $4
xxd -p $3 | tr -d "\n" >> $4
echo "');" >> $4