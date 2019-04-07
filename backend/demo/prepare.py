import sys
import os
sys.path.append(os.path.join(os.path.dirname(__file__), ".."))
from lib import db
from lib.api.cron import check_open_markets
from tools.create_market import insert_market_data
from tools.create_user import insert_user_data

from data import demo_users, demo_market


def main():
  db_url = os.environ["DB_URL"]
  conn = db.connect(db_url)

  market = demo_market()
  insert_market_data(conn, market)
  print("Prepared market data")

  users = demo_users()
  for user in users:
    insert_user_data(conn, user)
  print("Prepared user datas")

  conn.commit()

  print("Opening market")
  check_open_markets(db_url)

main()
