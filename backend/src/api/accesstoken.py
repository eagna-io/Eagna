from datetime import datetime, timezone
import db
from access_token import create_access_token, check_access_token
from api import response

class AccessTokenResource():
  def __init__(self, db_url):
    self.db_url = db_url

  def on_post(self, req, resp):
    email = req.media.get("email")
    hashed_pass = req.media.get("pass")
    if email == None or hashed_pass == None:
      resp.body = response.failure("parameter is not enough")
      return

    with db.connect(self.db_url) as conn:
      user_id = check_password(email, hashed_pass, conn)
      if user_id == None:
        resp.body = response.failure("invalid email or password")
        return

      # AccessToken の発行
      access_token = create_access_token(conn, user_id)

      resp.body = response.success(access_token)
      return

  def on_get(self, req, resp, access_token):
    with db.connect(self.db_url) as conn:
      user_id = check_access_token(conn, access_token)
      if user_id == None:
        resp.body = response.failure("access token is invalid")
        return

      resp.body = response.success({
        "userId": user_id,
      })

# Return "user_id" if success, or None
def check_password(email, hashed_pass, conn):
  sql = (
    "SELECT id FROM users "
    "WHERE email = %s "
    " AND hashed_pass = %s"
  )
  return db.query_one(conn, sql, (email, hashed_pass))
