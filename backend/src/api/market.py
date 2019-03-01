from api import response
import db
import market

class MarketResource():
  def on_get(self, req, resp, market_id):
    with db.connect_with_env() as conn:
      market = query_market(conn, market_id)
      if market == None:
        resp.body = response.failure("market not found")
        return

      res_data = to_dict(market)

      res_data["tokens"] = market.query_current_distribution(market_id, conn)

      if res_data["status"] == "settled":
        settlemment_token_id = market.query_settlement_token(market_id, conn)

      # access_token が指定されていた場合、ユーザー固有の情報も返す
      access_token = req.params.get("access_token")
      if access_token != None:
        user_id = check_access_token(access_token)
        if user_id == None:
          resp.body = response.failure("invalid access_token")
          return
        else:
          me_data = get_user_specific_data(user_id, res_data)
          res_data["me"] = me_data

      resp.body = response.success(res_data)


def query_market(conn, market_id):
  sql = (
    "SELECT "
    " id, title, organizer, short_desc, description,"
    " open_time, close_time, initial_coin_issue, status "
    "FROM markets "
    "WHERE markets.id = %s"
  )
  return db.query_one(conn, sql, (market_id,))


def to_dict(market):
  (id, title, organizer, short_desc, description, open_time, close_time, initial_coin_issue, status) = market
  return {
    "id": id,
    "title": title,
    "organizer": organizer,
    "short_desc": short_desc,
    "desc": description,
    "open_time": open_time,
    "close_time": close_time,
    "initial_coin_issue": initial_coin_issue,
    "status": status,
  }

def get_user_specific_data(market_id, user_id, conn):
  coins = market.query_user_coins(market_id, user_id, conn)
  tokens = market.query_user_tokens(market_id, user_id, conn)
  return {
    "coins": coins,
    "tokens": tokens,
  }
