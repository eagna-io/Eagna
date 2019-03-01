PG_DATA=pgdata
PG_USER=postgres
PG_PASS=postgres
PG_PORT=5555
PG_URI=postgres://$(PG_USER):$(PG_PASS)@localhost:$(PG_PORT)/$(PG_DATA)
DOCKER_PG_NAME=rohan-pg

install:
	pip install psycopg2 psycopg2-binary falcon

start-db:
	docker run -d --rm -e POSTGRES_DB=$(PG_DATA) -e POSTGRES_PASSWORD=$(PG_PASS) -e POSTGRES_USER=$(PG_USER) --name $(DOCKER_PG_NAME) -p $(PG_PORT):5432 postgres

stop-db:
	docker stop $(DOCKER_PG_NAME) || true

restart-db: stop-db start-db

# Run sql file specified by $SQL variable
run-sql-on-db:
	docker run -it --rm -v $(CURDIR)/$(SQL):/root/$(SQL) -e PGPASSWORD=$(PG_PASS) --link $(DOCKER_PG_NAME):$(DOCKER_PG_NAME) postgres psql -h $(DOCKER_PG_NAME) -U $(PG_USER) -d $(PG_DATA) -f /root/$(SQL)

initialize-db:
	$(MAKE) run-sql-on-db SQL=initialize.sql

tests/data/test_user.sql: tests/data/test_user.csv
	tests/data/csv2sql.sh $< > $@

tests/data/test_data.sql: tests/data/test_user.sql tests/data/test_raw_data.sql
	cat $^ > $@

connect-db:
	docker run -it --rm -e PGPASSWORD=$(PG_PASS) --link $(DOCKER_PG_NAME):$(DOCKER_PG_NAME) postgres psql -h $(DOCKER_PG_NAME) -U $(PG_USER) -d $(PG_DATA)

echo-db-uri:
	echo $(PG_URI)

prepare-test: tests/data/test_data.sql
	$(MAKE) restart-db
	sleep 5
	$(MAKE) initialize-db
	$(MAKE) run-sql-on-db SQL=$<

test:
	DATABASE_URL=$(PG_URI) python3 -m unittest discover tests

run:
	DATABASE_URL=$(PG_URI) python3 src/main.py
