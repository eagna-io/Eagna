#!/bin/bash

if [ $# -ne 1 ]; then
  echo "Please specify csv file"
  exit 1
fi

CSV_FILE=$1

while read row; do
  name=`echo ${row} | cut -d , -f 1`
  raw_pass=`echo ${row} | cut -d , -f 2`
  hashed_pass=`echo -n ${raw_pass} | openssl dgst -sha256`
  echo "INSERT INTO users (name, hashed_pass) VALUES ('${name}', '${hashed_pass}');"
done < ${CSV_FILE}
