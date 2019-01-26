import json
import os
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
  user.coins = coin_per_user
  db.update_user_coins(user)

# API server の起動
bind_host = os.getenv('BIND_HOST', '127.0.0.1')
bind_port = os.getenv('BIND_PORT', 8000)
print(f"Server start on {bind_host}:{bind_port}")
server = Server(db, bind_host, bind_port)
server.serve_forever()
