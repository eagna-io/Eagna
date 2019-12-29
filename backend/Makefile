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
