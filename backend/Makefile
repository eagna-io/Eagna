PG_DATA=pgdata
PG_USER=postgres
PG_PASS=postgres
PG_PORT=5555
PG_URL=postgres://$(PG_USER):$(PG_PASS)@localhost:$(PG_PORT)/$(PG_DATA)
DOCKER_PG_NAME=rohan-pg

install:
	pip install -r requirements.txt

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

connect-db:
	docker run -it --rm -e PGPASSWORD=$(PG_PASS) --link $(DOCKER_PG_NAME):$(DOCKER_PG_NAME) postgres psql -h $(DOCKER_PG_NAME) -U $(PG_USER) -d $(PG_DATA)

echo-db-uri:
	echo $(PG_URL)

run-demo:
	$(MAKE) restart-db
	sleep 2
	$(MAKE) initialize-db
	DB_URL=${PG_URL} python3 demo/prepare.py
	DB_URL=$(PG_URL) python3 main.py

test:
	DB_URL=$(PG_URL) python3 -m unittest discover tests
