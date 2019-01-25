from . import response

class MeResource():
  def __init__(self, db):
    self.db = db

  def on_get(self, req, resp):
    access_token = req.params.get("access_token")
    if access_token == None:
      resp.body = response.failure("access token is required")
      return

    user = self.db.check_access_token(access_token)
    if user == None:
      resp.body = response.failure("invalid access token")
      return

    resp.body = response.success(user_to_response(user))
    return

def user_to_response(user):
  return {
    "name": user.name,
    "hold_coin": user.hold_coin,
  }
