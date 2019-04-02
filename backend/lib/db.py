import os
import psycopg2

def connect_with_env():
  db_url = os.environ['DB_URL']
  return connect(db_url)

def connect(db_url):
  return psycopg2.connect(db_url)

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
