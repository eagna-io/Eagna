from datetime import datetime, timezone
from . import response

class LoginResource():
  def __init__(self, db):
    self.db = db

  def on_get(self, req, resp):
    user_name = req.params.get("user")
    hashed_pass = req.params.get("pass")
    if user_name == None or hashed_pass == None:
      resp.body = response.failure("parameter is not enough")
      return

    if self.db.check_login(user_name, hashed_pass) == False:
      resp.body = response.failure("invalid user or password")
      return

    now = int(datetime.now(timezone.utc).timestamp())
    access_token = self.db.create_access_token(user_name, now)
    self.db.commit()

    resp.body = response.success(access_token)
    return
