import os
import psycopg2

def connect_with_env():
  uri = os.environ.get('DATABASE_URL')
  return connect_with_uri(uri)

def connect_with_uri(uri):
  return psycopg2.connect(uri)

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

def update(conn, sql, args=()):
  with conn.cursor() as cur:
    cur.execute(sql, args)
