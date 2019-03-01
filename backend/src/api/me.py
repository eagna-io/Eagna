import api.response
from access_token import check_access_token

class MeResource():
  def on_get(self, req, resp):
    access_token = req.params.get("access_token")
    if access_token == None:
      resp.body = response.failure("access_token is required")
      return

    with db.connect_with_env() as conn:
      user_id = check_access_token(access_token, conn)
      if user_id == None:
        resp.body = response.failure("invalid access token")
        return

      user = query_user(user_id, conn)
      resp.body = response.success(user_to_response(user))
      return

# Return tuple of user information.
def query_user(user_id, conn):
  return db.query_one(conn, 'SELECT * FROM users WHERE id = %s', (user_id,))

def user_to_response(user):
  (id, name, email) = user
  return {
    "id": id,
    "name": user.name,
    "email": email,
  }
