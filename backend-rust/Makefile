PG_URL:=postgres://postgres:postgres@localhost:5432/postgres
REDIS_URL:=redis://localhost:6379

run-dev:
	docker stop postgres redis || true
	docker run -d --rm --name postgres -p 5432:5432 postgres
	docker run -d --rm --name redis -p 6379:6379 redis redis-server --bind 0.0.0.0
	sleep 2
	diesel migration run --database-url "${PG_URL}"
	RUST_LOG=warn,librohan=debug,rohan=debug \
	PG_URL="${PG_URL}" \
	REDIS_URL="${REDIS_URL}" \
	BIND="localhost:8080" \
	cargo run || true
	docker stop postgres redis || true


TMP_PG_URL:=postgres://postgres:postgres@localhost:5430/postgres

print_schema:
	docker run -d --rm --name postgres-tmp -p 5430:5432 postgres
	sleep 2
	diesel migration run --database-url "${TMP_PG_URL}" || true
	diesel print-schema --database-url "${TMP_PG_URL}" || true
	docker stop postgres-tmp


TEST_PG_URL:=postgres://postgres:postgres@localhost:5431/postgres
TEST_REDIS_URL:=redis://localhost:6378

test:
	docker run -d --rm --name postgres-test -p 5431:5432 postgres
	docker run -d --rm --name redis-test -p 6378:6379 redis redis-server --bind 0.0.0.0
	sleep 2
	diesel migration run --database-url "${TEST_PG_URL}"
	PG_URL="${TEST_PG_URL}" cargo run --example feed-test-data || true
	PG_URL="${TEST_PG_URL}" REDIS_URL="${TEST_REDIS_URL}" cargo test || true
	docker stop postgres-test redis-test
