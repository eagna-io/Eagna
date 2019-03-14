import db
from data import feed_user_data, feed_market_data
from market_observer import check_open_markets


def main():
  conn = db.connect_with_env()
  feed_user_data(conn)
  market_id = feed_market_data(conn)
  conn.commit()
  print("Feed data")

  print("Opening market")
  check_open_markets()

main()
