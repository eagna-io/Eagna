from lib.api import response
from lib.access_token import check_access_token
from lib import db

class MeResource():
  def __init__(self, db_url):
    self.db_url = db_url

  def on_get(self, req, resp):
    access_token = req.params.get("access_token")
    if access_token == None:
      resp.body = response.failure("access_token is required")
      return

    with db.connect(self.db_url) as conn:
      user_id = check_access_token(conn, access_token)
      if user_id == None:
        resp.body = response.failure("invalid access token")
        return

      me = query_user(conn, user_id)
      me["markets"] = query_markets(conn, user_id)

      resp.body = response.success(me)
      return

# Return tuple of user information.
def query_user(conn, user_id):
  sql = (
    "SELECT id, name, email FROM users "
    "WHERE id = %s"
  )
  (id, name, email) = db.query_one(conn, sql, (user_id,))
  return {
    "id": id,
    "name": name,
    "email": email,
  }

def query_markets(conn, user_id):
  sql = (
    "SELECT markets.id, markets.title, markets.status FROM markets "
    "INNER JOIN orders "
    " ON markets.id = orders.market_id "
    "  AND orders.type = 'initial_supply'"
    "  AND orders.user_id = %s"
  )
  return [
    {
      "id": id,
      "title": title,
      "status": status,
    }
    for (id, title, status)
    in db.query_all(conn, sql, (user_id,))
  ]
