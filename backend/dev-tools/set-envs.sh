#!/bin/bash

if [ $# -ne 1 ]; then
  echo "please specify valid argument : [set, unset]" 1>&2
  exit 1
fi

set_envs() {
  echo 'export ROHAN_BIND_HOST="127.0.0.1"'
  echo 'export ROHAN_BIND_PORT=8000'
  echo 'export ROHAN_DB_HOST="localhost"'
  echo 'export ROHAN_DB_PORT=5555'
  echo 'export ROHAN_DB_USER="postgres"'
  echo 'export ROHAN_DB_PASS="postgres"'
}

unset_envs() {
  echo 'unset ROHAN_BIND_HOST'
  echo 'unset ROHAN_BIND_PORT'
  echo 'unset ROHAN_DB_HOST'
  echo 'unset ROHAN_DB_PORT'
  echo 'unset ROHAN_DB_USER'
  echo 'unset ROHAN_DB_PASS'
}

case $1 in
  "set"   )   set_envs ;;
  "unset" )   unset_envs ;;
esac
