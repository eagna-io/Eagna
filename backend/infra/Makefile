.PHONY: print_schema
TMP_PG_URL:=postgres://postgres:postgres@localhost:5430/postgres
SCHEMA_FILE:=src/pg/schema.rs
print_schema:
	docker stop postgres-print-schema || true
	docker run -d --rm --name postgres-print-schema -p 5430:5432 postgres
	sleep 5
	diesel migration run --database-url "${TMP_PG_URL}" || true
	diesel print-schema --database-url "${TMP_PG_URL}" > ${SCHEMA_FILE}
	docker stop postgres-print-schema

DEV_PG_URL:=postgres://postgres:postgres@localhost:5429/postgres
start_dev_pg:
	docker stop crop-develop-postgres || true
	docker run -d --rm --name crop-develop-postgres -p 5429:5432 postgres
	sleep 5
	diesel database setup --database-url "${DEV_PG_URL}"
	psql "${DEV_PG_URL}" -c "insert into admins (id, email, cred, salt) values ('bc721702-0f77-459b-9604-4499e5dbed7c', 'admin@crop-pm.com', E'\\x73A071D82D5523CC121C161EEBD23F80E0B81D43AB3404D8FF5C96A635A6590DF774DC3DC03122C8A2AFE77C2322A88001A3EDD665647D284A93AC7752AA6333', E'\\x3F2809B5397B54ADCD1D5E884372E0A2E6932B65DC2B7746CA61C508D8F19034767730122DA243D707290EA5882E82956899406D679903FD93BF8BEA307433AB');"

stop_dev_pg:
	docker stop crop-develop-postgres || true
