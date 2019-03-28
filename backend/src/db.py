import os
import psycopg2

def connect_with_env():
  db_host = os.enviton['ROHAN_DB_HOST']
  db_port = os.enviton['ROHAN_DB_PORT']
  db_name = os.enviton['ROHAN_DB_NAME']
  db_user = os.enviton['ROHAN_DB_USER']
  db_pass = os.enviton['ROHAN_DB_PASS']
  return connect(db_host, db_port, db_name, db_user, db_pass)

def connect(db_host, db_port, db_name, db_user, db_pass):
  return psycopg2.connect(
    database = db_name,
    host = db_host,
    port = db_port,
    user = db_user,
    password = db_pass
  )

def query_all(conn, sql, args=()):
  with conn.cursor() as cur:
    cur.execute(sql, args)
    return cur.fetchall()

def query_one(conn, sql, args=()):
  with conn.cursor() as cur:
    cur.execute(sql, args)
    return cur.fetchone()

def insert(conn, sql, args=()):
  with conn.cursor() as cur:
    cur.execute(sql, args)

def insert_and_fetch(conn, sql, args=()):
  with conn.cursor() as cur:
    cur.execute(sql, args)
    return cur.fetchone()

def update(conn, sql, args=()):
  with conn.cursor() as cur:
    cur.execute(sql, args)
