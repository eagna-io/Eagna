from . import response

class MeResource():
  def __init__(self, db, market):
    self.db = db
    self.market = market

  def on_get(self, req, resp):
    access_token = req.params.get("access_token")
    if access_token == None:
      resp.body = response.failure("access token is required")
      return

    user = self.db.check_access_token(access_token)
    if user == None:
      resp.body = response.failure("invalid access token")
      return

    resp.body = response.success(user_to_response(user, self.market))
    return

def user_to_response(user, market):
  return {
    "name": user.name,
    "coins": user.coins,
    "markets": [market.toDict()],
  }
