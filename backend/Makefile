PG_URL:=postgres://postgres:postgres@localhost:5432/postgres
REDIS_URL:=redis://localhost:6379

start-db:
	docker stop postgres redis || true
	docker run -d --rm --name postgres -p 5432:5432 postgres
	docker run -d --rm --name redis -p 6379:6379 redis redis-server --bind 0.0.0.0
	sleep 2
	diesel migration run --database-url "${PG_URL}"

run-dev:
	RUST_LOG=warn,libeagna=debug,eagna=debug \
	PG_URL="${PG_URL}" \
	REDIS_URL="${REDIS_URL}" \
	BIND="0.0.0.0:8080" \
	FIREBASE_API_KEY=$${FIREBASE_API_KEY} \
	ACCESS_ALLOW_HOSTS="*" \
	cargo run


TMP_PG_URL:=postgres://postgres:postgres@localhost:5430/postgres

print_schema:
	docker run -d --rm --name postgres-tmp -p 5430:5432 postgres
	sleep 5
	diesel migration run --database-url "${TMP_PG_URL}" || true
	diesel print-schema --database-url "${TMP_PG_URL}" || true
	docker stop postgres-tmp


test:
	bash tests/run.sh
