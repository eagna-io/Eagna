from datetime import datetime, timezone
import db
from access_token import create_access_token
import api.response

class LoginResource():
  def on_get(self, req, resp):
    email = req.params.get("email")
    hashed_pass = req.params.get("pass")
    if user_name == None or hashed_pass == None:
      resp.body = response.failure("parameter is not enough")
      return

    with db.connect_with_env() as conn:
      user_id = check_password(email, hashed_pass, conn)
      if user_id == None:
        resp.body = response.failure("invalid email or password")
        return

      # AccessToken の発行
      access_token = create_access_token(user_id, conn)

      resp.body = response.success(access_token)
      return

# Return "user_id" if success, or None
def check_password(email, hashed_pass, conn):
  sql = (
    "SELECT id FROM users "
    "WHERE email = %s "
    " AND hashed_pass = %s"
  )
  return db.query_one(conn, sql, (email, hashed_pass))
