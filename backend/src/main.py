import json
import os
from market import Market
from db import DB
from api.server import Server

# Market情報の読み込み
f = open('market_def.json', 'r')
market_defs = json.load(f)
market = Market.init_with_json(market_defs)

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
server = Server(db, market, bind_host, bind_port)
server.serve_forever()
