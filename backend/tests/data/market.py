import db

def insert_market_data(conn, market):
  sql = (
    "INSERT INTO markets "
    "( title, organizer, short_desc, description, open_time, close_time, "
    "  initial_coin_issue, status ) "
    "VALUES "
    "( %s, %s, %s, %s, %s, %s, %s, %s ) "
    "RETURNING id"
  )
  market_id = db.insert_and_fetch(conn, sql,
    (
      market["title"],
      market["organizer"],
      market["short_desc"],
      market["desc"],
      market["open_time"],
      market["close_time"],
      market["initial_coin_issue"],
      market["status"],
    )
  )[0]
  insert_tokens(conn, market_id, market["tokens"])
  return market_id


def insert_tokens(conn, market_id, tokens):
  sql = (
    "INSERT INTO market_tokens "
    "(name, description, market_id, initial_price) "
    "VALUES "
    "(%s, %s, %s, %s)"
  )
  for token in tokens:
    db.insert(conn, sql,
      (
        token["name"],
        token["desc"],
        market_id,
        token["initial_price"]
      )
    )



sample_tokens = [
  {
    "name": "Yuya",
    "desc": "古澤裕也",
    "initial_price": 50,
  },
  {
    "name": "Atsuki",
    "desc": "高橋篤樹",
    "initial_price": 50,
  },
]

sample_market = {
  "title": "スマブラ大会の勝者は？",
  "organizer": "RohanMarket.inc",
  "short_desc": "3/1に開催されるスマブラ大会の勝者を予想する",
  "desc": "3/1に開催されるスマブラ大会の勝者を予想する",
  "open_ts": 1552175249, # 2019-03-09 23:47:29+00:00
  "close_ts": 1552178849, # 2019-03-10 0:47:29+00:00
  "initial_coin_issue": 10000,
  "status": "preparing", # open_ts は過ぎているが、公開処理をしていないので preparing 状態
  "tokens": sample_tokens,
}


