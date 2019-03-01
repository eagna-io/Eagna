from hashlib import sha256
import db


def insert_sample_users(conn):
  insert_user_data("alice", "alice@rohanmarket.com", "alice")
  insert_user_data("bob", "bob@rohanmarket.com", "bob")


def insert_user_data(conn, name, email, raw_pass):
  hashed_pass = sha256(raw_pass.encode()).hexdigest()
  sql = (
   "INSERT INTO users (name, email, hashed_pass) "
   "VALUES (%s, %s, %s)"
  )
  db.insert(conn, sql, (name, email, hashed_pass))
