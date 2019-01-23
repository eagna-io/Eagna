import json
from rohan.market import Market
from rohan.db import DB
from rohan.api.server import Server

# Market情報の読み込み
f = open('market_def.json', 'r')
market_defs = json.load(f)
market = Market(
    title = market_defs["title"],
    desc = market_defs["desc"],
    opening_time = market_defs["opening_time"],
    closing_time = market_defs["closing_time"],
    outcomes = market_defs["outcomes"],
    initial_coin_issue = market_defs["coin_info"]["initial_coin_issue"]
)

db = DB.init_with_env()

# User へのコインの配布
users = db.query_users()
coin_per_user = market.initial_coin_issue / len(users)
for user in users:
  user.hold_coin = coin_per_user
  db.update_user_hold_coin(user)

# API server の起動
server = Server(db, "127.0.0.1", 8099)
server.serve_forever()
