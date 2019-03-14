from api import response
import db
from access_token import check_access_token
from market import query_current_distribution, query_settlement_token, query_user_coins, query_user_tokens

class MarketResource():
  def on_get(self, req, resp, id):
    market_id = id
    with db.connect_with_env() as conn:
      market = query_market(conn, market_id)
      if market == None:
        resp.body = response.failure("market not found")
        return

      res_data = market_to_dict(market)
      res_data["tokens"] = query_tokens(conn, market_id)

      if res_data["status"] == "settled":
        settlemment_token_id = query_settlement_token(conn, market_id)

      # access_token が指定されていない場合、一般的な情報のみ返す
      access_token = req.params.get("access_token")
      if access_token == None:
        resp.body = response.success(res_data)
        return

      # access_token が指定されていた場合、ユーザー固有の情報も返す
      user_id = check_access_token(access_token, conn)
      if user_id == None:
        resp.body = response.failure("invalid access_token")
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
    " initial_coin_issue, status "
    "FROM markets "
    "WHERE markets.id = %s"
  )
  return db.query_one(conn, sql, (market_id,))


def market_to_dict(market):
  (id, title, organizer, short_desc, description, open_ts, close_ts, initial_coin_issue, status) = market
  return {
    "id": id,
    "title": title,
    "organizer": organizer,
    "short_desc": short_desc,
    "desc": description,
    "open_ts": open_ts,
    "close_ts": close_ts,
    "initial_coin_issue": initial_coin_issue,
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
    "token_id": token_id,
    "amount": amount,
  }


def get_user_specific_data(conn, market_id, user_id):
  coins = query_user_coins(conn, market_id, user_id)
  tokens = [
    {
      "token_id": id,
      "amount": amount,
    }
    for (id, amount)
    in query_user_tokens(conn, market_id, user_id)
  ]
  return {
    "coins": coins,
    "tokens": tokens,
  }
