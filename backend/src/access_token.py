import secrets
import db

# AccessTokenの長さ
TOKEN_LENGTH = 8

# AccessTokenが有効な秒数
VALID_SECS = 60 * 60

# Return "token" or None
def create_access_token(user_id, conn):
  token = secrets.token_hex(TOKEN_LENGTH)
  sql = (
   "INSERT INTO access_tokens "
   "(token, user_id) "
   "VALUES (%s, %s)"
  )
  db.insert(conn, sql, (token, user_id))
  return token


# Return "user_id" or None
def check_access_token(token, conn):
  sql = (
   "SELECT user_id FROM access_tokens "
   "WHERE token = %s "
   " AND force_expired = False "
   " AND now() < created_at + INTERVAL '1 hour'"
  )
  return db.query_one(conn, sql, (token,))
