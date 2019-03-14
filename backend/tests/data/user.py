from hashlib import sha256
import db


def insert_user_data(conn, user):
  hashed_pass = sha256(user["raw_pass"].encode()).hexdigest()
  sql = (
   "INSERT INTO users (name, email, hashed_pass) "
   "VALUES (%s, %s, %s) "
   "RETURNING id"
  )
  return db.insert_and_fetch(conn, sql,
    (
      user["name"],
      user["email"],
      hashed_pass,
    )
  )[0]


def insert_sample_user_data(conn):
  alice_id = insert_user_data(conn, user_alice)
  bob_id = insert_user_data(conn, user_bob)
  return [alice_id, bob_id]


user_alice = {
  "name": "alice",
  "email": "alice@rohanmarket.com",
  "raw_pass": "alice",
}

user_bob = {
  "name": "bob",
  "email": "bob@rohanmarket.com",
  "raw_pass": "bob",
}
