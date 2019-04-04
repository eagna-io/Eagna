from hashlib import sha256

import sys
import os
sys.path.append(os.path.join(os.path.dirname(__file__), ".."))
from lib import db


def main():
  db_url = os.environ.get('DB_URL')
  if db_url == None:
    print("DB_URL environment variable is not set")
    return

  conn = db.connect(db_url)

  name = input("Enter user name : ")
  email = input("Enter email address : ")
  raw_pass = input("Enter password : ")
  user = {
    "name": name,
    "email": email,
    "raw_pass": raw_pass,
  }

  insert_user_data(conn, user)

  conn.commit()
  print(f"Success to create a new user [{name}]")
  return


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

if __name__ == "__main__":
  main()
