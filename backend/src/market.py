from datetime import datetime, timezone
import db

# Return array of tuple of outcome_id and amount
def query_current_distribution(market_id, conn):
  sql = (
    "SELECT orders.token_id, SUM(orders.amount_token) "
    "FROM orders "
    "GROUP BY orders.token_id "
    "WHERE orders.market_id = %s"
    " AND orders.token_id IS NOT NULL"
  )
  return db.query_all(conn, sql, (market_id,))


def query_user_coins(market_id, user_id, conn):
  sql = (
    "SELECT SUM(orders.amount_coin) "
    "FROM orders "
    "WHERE orders.user_id = %s "
    " AND orders.market_id = %s"
  )
  return db.query_one(conn, sql, (user_id, market_id))


def query_user_tokens(market_id, user_id, conn):
  sql = (
    "SELECT orders.token_id, SUM(orders.amount_token) "
    "FROM orders "
    "GROUP BY orders.token_id "
    "WHERE orders.market_id = %s "
    " AND orders.user_id = %s "
    " AND orders.token_id IS NOT NULL"
  )
  return db.query_all(conn, sql, (market_id, user_id))


def query_settlement_token(market_id, conn):
  sql = (
   "SELECT id FROM market_tokens "
   "WHERE is_settlement_token = True"
  )
  return db.query_one(conn, sql)
