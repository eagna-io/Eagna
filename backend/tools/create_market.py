import json

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

  if len(sys.argv) != 2:
    print("Usage : python %s [path/to/market.json]"%(sys.argv[0],))
    return

  f = open(sys.argv[1])
  market = json.load(f)

  market_id = insert_market_data(conn, market)
  conn.commit()
  print(f"Success to create market [{market_id}]")
  return


def insert_market_data(conn, market):
  sql = (
    "INSERT INTO markets "
    "( title, organizer, short_desc, description, open_time, close_time, status ) "
    "VALUES "
    "( %s, %s, %s, %s, %s, %s, %s ) "
    "RETURNING id"
  )
  market_id = db.insert_and_fetch(conn, sql,
    (
      market["title"],
      market["organizer"],
      market["shortDesc"],
      market["desc"],
      market["openTime"],
      market["closeTime"],
      market["status"],
    )
  )[0]
  insert_tokens(conn, market_id, market["tokens"])
  return market_id


def insert_tokens(conn, market_id, tokens):
  sql = (
    "INSERT INTO market_tokens "
    "(name, description, market_id, initial_price) "
    "VALUES "
    "(%s, %s, %s, %s)"
  )
  for token in tokens:
    db.insert(conn, sql,
      (
        token["name"],
        token["desc"],
        market_id,
        token["initialPrice"]
      )
    )


if __name__ == "__main__":
  main()
