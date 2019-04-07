from lib.api import response
from lib.market import query_current_distribution, query_user_coins
from lib.lmsr import cost
from lib.access_token import check_access_token
from lib import db

class OrderResource():
  def __init__(self, db_url):
    self.db_url = db_url

  def on_post(self, req, resp):
    access_token = req.media.get("accessToken")
    token_id = req.media.get("tokenId")
    amount_token = req.media.get("amountToken")
    amount_coin = req.media.get("amountCoin")

    if access_token == None or token_id == None or amount_token == None or amount_coin == None:
      resp.body = response.failure("parameter is not enough")
      return

    with db.connect(self.db_url) as conn:
      # access_token を検証
      user_id = check_access_token(conn, access_token)
      if user_id == None:
        resp.body = response.failure("access token is invalid")
        return

      # token_id からmarket を取得
      market_id = query_market_id(conn, token_id)
      if market_id == None:
        resp.body = response.failure("token id is invalid")
        return

      # マーケットがopen状態かチェック
      if check_market_status(conn, market_id) != "open":
        resp.body = response.failure("market is already closed")
        return


      # 各トークンの現在の流通量を取得
      # Never error
      cur_tokens = query_current_distribution(conn, market_id)

      new_distribution = [
        amount_token + amount if id == token_id else amount
        for (id, amount)
        in cur_tokens
      ]
      cur_distribution = [amount for (id, amount) in cur_tokens]

      # 対象のトークンを売却できるだけ保持しているかチェック
      target_user_token = query_target_user_token(conn, token_id, user_id)
      if target_user_token + amount_token < 0:
        resp.body = response.failure("you dont have the token enough")
        return

      # 対象のトークンを購入できるだけ資金を持っているかチェック
      user_coins = query_user_coins(conn, market_id, user_id)
      if user_coins + amount_coin < 0:
        resp.body = response.failure("you don't have enough coin")
        return

      # amount_coin が適切かチェック
      expected_amount_coin = cost(cur_distribution) - cost(new_distribution)
      if amount_coin != expected_amount_coin:
        resp.body = response.failure("amount coin is invalid")
        return

      # DB にorder を記録
      save_order(user_id, market_id, token_id, amount_token, amount_coin, conn)
      
      resp.body = response.success("success")


def query_market_id(conn, token_id):
  sql = (
    "SELECT market_id FROM market_tokens "
    "WHERE id = %s"
  )
  return db.query_one(conn, sql, (token_id,))


def check_market_status(conn, market_id):
  sql = (
    "SELECT status FROM market "
    "WHERE id = %s"
  )
  return db.query_one(conn, sql, (market_id,))


def query_target_user_token(conn, token_id, user_id):
  sql = (
    "SELECT COALESCE(SUM(amount_token), 0) FROM orders "
    "WHERE token_id = %s and user_id = %s"
  )
  return db.query_one(conn, sql, (token_id, user_id))[0]


# Return True if success, otherwise False
def save_order(user_id, market_id, token_id, amount_token, amount_coin, conn):
  sql = (
    "INSERT INTO orders "
    "(user_id, market_id, token_id, amount_token, amount_coin, type) "
    "VALUES (%s, %s, %s, %s, %s, 'normal')"
  )
  return db.insert(conn, sql, (user_id, market_id, token_id, amount_token, amount_coin))
