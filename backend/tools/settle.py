import psycopg2

token_id = 0
market_id = 0

db_uri = os.environ.get('DATABASE_URL')

with psycopg2.connect(db_uri) as conn:
  insert_reword_orders(conn, token_id)
  update_market_tokens(conn, token_id)


# Insert reword orders
def insert_reword_orders(conn, token_id):
  for (user_id, amount_token) in query_user_token(conn, token_id):
    insert_reword_order(conn, user_id, market_id, token_id, amount_token)


def query_user_token(conn, token_id):
  sql = (
   "SELECT user_id, SUM(amount_token) FROM orders "
   "GROUP BY user_id "
   "WHERE token_id = %s"
  )
  with conn.cursor() as cur:
    cur.execute(sql, (token_id,))
    return cur.fetchall()


def insert_reword_order(conn, user_id, market_id, token_id, amount_token)
  sql = (
   "INSERT INTO orders "
   "(user_id, market_id, token_id, amount_token, amount_coin, type) "
   "VALUES "
   "( %s, %s, %s, %s, %s, 'reword' )"
  )
  with conn.cursor() as cur:
    cur.execute(sql, (user_id, market_id, token_id, -amount_token, amount_token))



# Update market_tokens
def update_market_tokens(conn, token_id):
  sql = (
   "UPDATE market_tokens SET is_settlement_token = True "
   "WHERE id = %s"
  )
  with conn.cursor() as cur:
    cur.execute(sql, (token_id,))
