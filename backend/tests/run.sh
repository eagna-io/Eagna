#!/bin/bash
set -e

TEST_SERVER_PID_FILE="tests/server.pid"

# 前回正常終了しなかった場合の処理
docker stop postgres-test redis-test || true
if [ -e "${TEST_SERVER_PID_FILE}" ]; then
  kill -9 $(cat ${TEST_SERVER_PID_FILE})
  rm -f ${TEST_SERVER_PID_FILE}
fi

# DBの初期化
docker run -d --rm --name postgres-test -p 5431:5432 postgres
docker run -d --rm --name redis-test -p 6378:6379 redis redis-server --bind 0.0.0.0

sleep 2

export PG_URL="postgres://postgres:postgres@localhost:5431"
export REDIS_URL="redis://localhost:6378"

diesel migration run --database-url "${PG_URL}"
psql -f "tests/init_data.sql" "${PG_URL}"

# サーバーの起動
export BIND="0.0.0.0:8081"
export RUN_MODE="test"
if [ -z "${TEST_VERBOSE}" ]; then
  export RUST_LOG="warn"
else
  export RUST_LOG="warn,libeagna=debug,eagna=debug"
  export RUST_BACKTRACE=1
fi

cargo build
cargo run &
server_pid=$!
echo ${server_pid} > ${TEST_SERVER_PID_FILE}

sleep 2

echo "running test scenario..."

# シナリオテスト
python3 tests/scenario_01.py

kill -9 ${server_pid}
rm -f ${TEST_SERVER_PID_FILE}
