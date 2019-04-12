from time import sleep
from datetime import datetime, timezone
import falcon
from lib import db

class CronResource():
  def __init__(self, db_url):
    self.db_url = db_url

  def on_get(self, req, resp):
    validate_cron_req(req)

    check_open_markets(self.db_url)
    check_close_markets(self.db_url)

    resp.status = falcon.HTTP_200
    return


# "X-Appengine-Cron" ヘッダーがあるかチェック。
# ない場合は HTTPBadRequest 例外が飛ぶ
# 値まではチェックしない
def validate_cron_req(req):
  req.get_header("X-Appengine-Cron", required = True)


# Opening instruction

INITIAL_COIN_SUPPLY = 10000

def check_open_markets(db_url):
  with db.connect(db_url) as conn:
    # 新しくopen する必要のあるmarket 一覧を取得
    new_markets = query_new_open_markets(conn)
    if len(new_markets) != 0:
      print("Open %s new markets"%(len(new_markets),))
      # market に参加するuser 一覧を取得（現在は全員）
      users = query_users(conn)
      for (market_id,) in new_markets:
        # 参加者にコインを配布
        distribute_init_coin(market_id, users, conn)
        open_market(market_id, conn)


def query_new_open_markets(conn):
  sql = (
    "SELECT id FROM markets "
    "WHERE open_time <= now()"
    " AND status = 'preparing'"
  )
  return db.query_all(conn, sql)


def query_users(conn):
  return db.query_all(conn, "SELECT id FROM users")


def distribute_init_coin(market_id, users, conn):
  sql = (
    "INSERT INTO orders "
    "(user_id, market_id, amount_token, amount_coin, type) "
    "VALUES "
    "(%s, %s, 0, %s, 'initial_supply')"
  )
  for user_id in users:
    db.insert(conn, sql, (user_id, market_id, INITIAL_COIN_SUPPLY))


def open_market(market_id, conn):
  db.update(conn, "UPDATE markets SET status = 'open' WHERE id = %s", (market_id,))



# Closing instruction

def check_close_markets(db_url):
  with db.connect(db_url) as conn:
    new_markets = query_new_close_markets(conn)
    if len(new_markets) != 0:
      print("Close %s new markets"%(len(new_markets),))
      for market_id in new_markets:
        # market_status をclosed に変更
        # settlement tokenの決定、reward の配布は手動で行う
        close_market(market_id, conn)


def query_new_close_markets(conn):
  sql = (
    "SELECT id FROM markets "
    "WHERE close_time <= now() "
    " AND status = 'open'"
  )
  return db.query_all(conn, sql)


def close_market(market_id, conn):
  db.update(conn, "UPDATE markets SET status = 'closed' WHERE id = %s", (market_id,))
