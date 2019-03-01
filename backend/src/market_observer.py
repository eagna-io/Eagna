from time import sleep
from datetime import datetime, timezone
import db

def observe_market():
  while True:
    print("Check markets...")
    check_open_markets()
    check_close_markets()
    sleep_until_next_minite()


# Opening instruction

def check_open_markets():
  with db.connect_with_env() as conn:
    # 新しくopen する必要のあるmarket 一覧を取得
    new_markets = query_new_open_markets(conn)
    if len(new_markets) != 0:
      n = len(new_markets)
      print(f"Open {n} new markets")
      # market に参加するuser 一覧を取得（現在は全員）
      users = query_users(conn)
      for (market_id, init_coin) in new_markets:
        # 参加者にコインを配布
        distribute_init_coin(market_id, init_coin, users, conn)
        open_market(market_id, conn)


def query_new_open_markets(conn):
  cur_ts = current_ts()
  sql = (
    "SELECT id, initial_coin_issue FROM markets "
    "WHERE open_ts <= %s"
    " AND status = 'preparing'"
  )
  return db.query_all(conn, sql, (cur_ts,))


def query_users(conn):
  return db.query_all(conn, "SELECT id FROM users")


def distribute_init_coin(market_id, init_coin, users, conn):
  init_coin_per_user = int(init_coin / len(users))
  sql = (
    "INSERT INTO orders "
    "(user_id, market_id, amount_token, amount_coin, type) "
    "VALUES "
    "(%s, %s, 0, %s, 'initial_supply')"
  )
  for user_id in users:
    db.insert(conn, sql, (user_id, market_id, init_coin_per_user))


def open_market(market_id, conn):
  db.update(conn, "UPDATE markets SET status = 'open' WHERE id = %s", (market_id,))



# Closing instruction

def check_close_markets():
  with db.connect_with_env() as conn:
    new_markets = query_new_close_markets(conn)
    if len(new_markets) != 0:
      for market_id in new_markets:
        # market_status をclosed に変更
        # settlement tokenの決定、reward の配布は手動で行う
        close_market(market_id, conn)


def query_new_close_markets(conn):
  cur_ts = current_ts()
  sql = (
    "SELECT id FROM markets "
    "WHERE close_ts <= %s "
    " AND status = 'open'"
  )
  return db.query_all(conn, sql, (cur_ts,))


def close_market(market_id, conn):
  db.update(conn, "UPDATE markets SET status = 'closed' WHERE id = %s", (market_id,))



def sleep_until_next_minite():
  cur_ts = current_ts()
  next_ts = ((cur_ts / 60) + 1) * 60
  sleep(next_ts - cur_ts)


def current_ts():
  return int(datetime.now(timezone.utc).timestamp())