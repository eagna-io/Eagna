import json
from rohan.market import Market
from rohan.db import DB
from rohan.api.server import Server

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
server = Server(db, "127.0.0.1", 8099)
server.serve_forever()
