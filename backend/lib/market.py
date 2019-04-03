from lib import db

# Return array of (outcome_id, amount)
def query_current_distribution(conn, market_id):
  sql = (
    "SELECT market_tokens.id, COALESCE(SUM(orders.amount_token), 0) "
    "FROM market_tokens "
    "LEFT OUTER JOIN orders "
    " ON market_tokens.id = orders.token_id "
    "WHERE market_tokens.market_id = %s"
    "GROUP BY market_tokens.id"
  )
  return db.query_all(conn, sql, (market_id,))


def query_user_coins(conn, market_id, user_id):
  sql = (
    "SELECT COALESCE(SUM(amount_coin), 0) "
    "FROM orders "
    "WHERE user_id = %s "
    " AND market_id = %s"
  )
  return db.query_one(conn, sql, (user_id, market_id))[0]


def query_user_tokens(conn, market_id, user_id):
  sql = (
    "SELECT market_tokens.id, COALESCE(SUM(orders.amount_token), 0) "
    "FROM market_tokens "
    "LEFT OUTER JOIN orders "
    " ON market_tokens.id = orders.token_id "
    "  AND orders.user_id = %s "
    "WHERE market_tokens.market_id = %s "
    "GROUP BY market_tokens.id"
  )
  return db.query_all(conn, sql, (user_id, market_id))


def query_settlement_token(conn, market_id):
  sql = (
   "SELECT id FROM market_tokens "
   "WHERE is_settlement_token = True"
   " AND market_id = %s"
   )
  return db.query_one(conn, sql, (market_id,))[0]