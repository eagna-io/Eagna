import sys
import os
sys.path.append(os.path.join(os.path.dirname(__file__), ".."))
from lib import db

def main():
  conn = db.connect_with_env()

  print("Markets waiting for settle")
  closed_markets = query_closed_markets(conn)
  if len(closed_markets) == 0:
    print("No market is waiting for settle")
    return

  for (market_id, market_title) in query_closed_markets(conn):
    print(f"{market_id}  {market_title}")

  settle_market_id = input("Enter market id : ")

  settle_market_tokens = query_tokens(conn, settle_market_id)
  print("Tokens")
  for (token_id, token_name) in settle_market_tokens:
    print(f"{token_id}  {token_name}")

  settle_token_id = input("Enter token id : ")

  insert_settlement_orders(conn, settle_market_id, settle_token_id)
  set_settle_token_id(conn, settle_market_id, settle_token_id)

  conn.commit()
  print("Success to settle!")


def query_closed_markets(conn):
  sql = (
    "SELECT id, title FROM markets "
    "WHERE status = 'closed'"
  )
  return db.query_all(conn, sql)

def query_tokens(conn, market_id):
  sql = (
    "SELECT id, name FROM market_tokens "
    "WHERE market_id = %s"
  )
  return db.query_all(conn, sql, (market_id,))

# 'reward' order と 'failure' order を履歴に追加する。
# -> Userの保有tokenを全てcoinに変換する
def insert_settlement_orders(conn, settle_market_id, settle_token_id):
  # Insert reward orders
  user_tokens = query_user_token(conn, settle_market_id)
  for (user_id, token_id, amount_token) in user_tokens:
    if token_id == settle_token_id:
      insert_reward_order(conn, user_id, settle_market_id, token_id, amount_token)
    else:
      insert_failure_order(conn, user_id, settle_market_id, token_id, amount_token)


def query_user_token(conn, market_id):
  sql = (
   "SELECT user_id, token_id, SUM(amount_token) FROM orders "
   "WHERE market_id = %s"
   "GROUP BY user_id, token_id "
  )
  return db.query_all(conn, sql, (market_id,))


def insert_reward_order(conn, user_id, market_id, token_id, amount_token):
  sql = (
   "INSERT INTO orders "
   "(user_id, market_id, token_id, amount_token, amount_coin, type) "
   "VALUES "
   "( %s, %s, %s, %s, %s, 'reward' )"
  )
  db.insert(conn, sql, (user_id, market_id, token_id, -amount_token, amount_token))

def insert_failure_order(conn, user_id, market_id, token_id, amount_token):
  sql = (
   "INSERT INTO orders "
   "(user_id, market_id, token_id, amount_token, amount_coin, type) "
   "VALUES "
   "( %s, %s, %s, %s, %s, 'failure' )"
  )
  db.insert(conn, sql, (user_id, market_id, token_id, -amount_token, 0))


# Update market_tokens
def set_settle_token_id(conn, market_id, token_id):
  sql = (
   "UPDATE markets "
   "SET settle_token_id = %s, status = 'settled' "
   "WHERE id = %s"
  )
  db.update(conn, sql, (token_id, market_id))


if __name__ == "__main__":
  main()
