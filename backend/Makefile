.PHONY: build-docker
build-docker:
	docker build -t crop-backend --build-arg BUILD_MODE=debug --rm=false .

.PHONY: start-db
PG_URL:=postgres://postgres:postgres@localhost:5432/postgres
REDIS_URL:=redis://localhost:6379
start-db:
	docker stop postgres redis || true
	docker run -d --rm --name postgres -p 5432:5432 postgres
	docker run -d --rm --name redis -p 6379:6379 redis redis-server --bind 0.0.0.0
	sleep 2
	diesel migration run --database-url "${PG_URL}"
	psql -f "tests/init_data.sql" "${PG_URL}"

.PHONY: run-dev
run-dev:
	docker run --rm \
	-e RUST_LOG=warn,libeagna=debug,eagna=debug \
	-e PG_URL="${PG_URL}" \
	-e REDIS_URL="${REDIS_URL}" \
	-e PORT="8080" \
	-e ACCESS_ALLOW_HOSTS="*" \
	-p 8080:8080 \
	crop-backend


.PHONY: print_schema
TMP_PG_URL:=postgres://postgres:postgres@localhost:5430/postgres
SCHEMA_FILE:=src/infra/postgres/schema.rs
print_schema:
	docker stop postgres-print-schema || true
	docker run -d --rm --name postgres-print-schema -p 5430:5432 postgres
	sleep 5
	diesel migration run --database-url "${TMP_PG_URL}" || true
	diesel print-schema --database-url "${TMP_PG_URL}" > ${SCHEMA_FILE}
	docker stop postgres-print-schema


.PHONY: test
test:
	bash tests/run-all.sh

.PHONY: print_sample_credential
print_sample_credential:
	cargo run --example print_sample_credential
