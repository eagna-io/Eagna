from math import floor
from api import response
from market import query_current_distribution
import db

class OrderResource():
  def on_post(self, req, resp):
    access_token = req.media.get("access_token")
    token_id = req.media.get("token_id")
    amount_token = req.media.get("amount_token")
    amount_coin = req.media.get("amount_coin")

    if access_token == None or token_id == None or amount_token == None or amount_coin == None:
      resp.body = response.failure("parameter is not enough")
      return

    with db.connect_with_env() as conn:
      # access_token を検証
      user_id = check_access_token(access_token, conn)
      if user_id == None:
        resp.body = response.failure("access_token is invalid")
        return

      # token_id からmarket を取得
      market_id = query_market_id(token_id, conn)
      if market_id == None:
        resp.body = response.failure("token_id is invalid")
        return

      # 各トークンの現在の流通量を取得
      # Never error
      cur_tokens = query_current_distribution(market_id, conn)

      new_distribution = [
        amount_token + amount if id == token_id else amount
        for (id, amount)
        in cur_tokens
      ]
      cur_distribution = [amount for (id, amount) in cur_tokens]

      # 対象のトークンが売却できるだけ流通しているかチェック
      if min(new_distribution) < 0:
        resp.body = response.failure("the token is not distributed enough")
        return

      # 対象のトークンを購入できるだけ資金を持っているかチェック
      user_coins = query_user_coins(user_id, market_id, conn)
      if user_coins < amount_coin:
        resp.body = response.failure("you don't have enough coin")
        return

      # amount_coin が適切かチェック
      expected_amount_coin = floor(cost(new_distribution) - cost(cur_distribution))
      if amount_coin != expected_amount_coin:
        resp.body = response.failure("amount_coin is not valid")
        return

      # DB にorder を記録
      save_order(user_id, market_id, token_id, amount_token, amount_coin, conn)
      
      resp.body = response.success("success")


def query_market_id(token_id, db):
  return db.query_one('SELECT market_id FROM market_outcomes WHERE id = %s', (token_id,))


# Return True if success, otherwise False
def save_order(user_id, market_id, token_id, amount_token, amount_coin, conn):
  sql = (
    "INSERT INTO orders "
    "(user_id, market_id, token_id, amount_token, amount_coin, type) "
    "VALUES (%s, %s, %s, %s, %s, 'normal')"
  )
  return db.insert(conn, sql, (user_id, market_id, token_id, amount_token, amount_coin))
