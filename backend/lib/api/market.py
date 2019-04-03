from lib.api import response
from lib import db
from lib.access_token import check_access_token
from lib.market import query_current_distribution, query_settlement_token, query_user_orders

class MarketResource():
  def __init__(self, db_url):
    self.db_url = db_url

  def on_get(self, req, resp, id):
    market_id = id
    with db.connect(self.db_url) as conn:
      market = query_market(conn, market_id)
      if market == None:
        resp.body = response.failure("market not found")
        return

      res_data = market_to_dict(market)
      res_data["tokens"] = query_tokens(conn, market_id)

      if res_data["status"] == "settled":
        res_data["settlementTokenId"] = query_settlement_token(conn, market_id)

      # access_token が指定されていない場合、一般的な情報のみ返す
      access_token = req.params.get("access_token")
      if access_token == None:
        resp.body = response.success(res_data)
        return

      # access_token が指定されていた場合、ユーザー固有の情報も返す
      user_id = check_access_token(conn, access_token)
      if user_id == None:
        resp.body = response.failure("invalid access token")
        return

      me_data = get_user_specific_data(conn, market_id, user_id)
      res_data["me"] = me_data

      resp.body = response.success(res_data)
      return


def query_market(conn, market_id):
  sql = (
    "SELECT "
    " id, title, organizer, short_desc, description,"
    " EXTRACT(EPOCH FROM open_time), "
    " EXTRACT(EPOCH FROM close_time), "
    " start_coin_supply, status "
    "FROM markets "
    "WHERE markets.id = %s"
  )
  return db.query_one(conn, sql, (market_id,))


def market_to_dict(market):
  (id, title, organizer, short_desc, description, open_ts, close_ts, start_coin_supply, status) = market
  return {
    "id": id,
    "title": title,
    "organizer": organizer,
    "shortDesc": short_desc,
    "desc": description,
    "openTs": open_ts,
    "closeTs": close_ts,
    "startCoinSupply": start_coin_supply,
    "status": status,
  }


def query_tokens(conn, market_id):
  sql = (
    "SELECT id, name, description FROM market_tokens "
    "WHERE market_id = %s"
  )
  sql = (
    "SELECT "
    " market_tokens.id, "
    " market_tokens.name, "
    " market_tokens.description, "
    " COALESCE(SUM(orders.amount_token), 0) "
    "FROM market_tokens "
    "LEFT OUTER JOIN orders "
    " ON market_tokens.id = orders.token_id "
    "WHERE market_tokens.market_id = %s"
    "GROUP BY market_tokens.id"
  )
  return [token_to_dict(t) for t in db.query_all(conn, sql, (market_id,))]


def token_to_dict(token):
  (id, name, desc, amount) = token
  return {
    "id": id,
    "name": name,
    "desc": desc,
    "amount": amount,
  }


def distribution_to_dict(distribution):
  (token_id, amount) = distribution
  return {
    "tokenId": token_id,
    "amount": amount,
  }


def get_user_specific_data(conn, market_id, user_id):
  orders = [ order_to_dict(order) for order
    in query_user_orders(conn, market_id, user_id) ]
  return {
    "orders": orders,
  }

def order_to_dict(order):
  (id, token_id, amount_token, amount_coin, type, time) = order
  return {
      "id": id,
      "tokenId": token_id,
      "amountToken": amount_token,
      "amountCoin": amount_coin,
      "type": type,
      "time": time,
  }
