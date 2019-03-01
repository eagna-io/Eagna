import secrets
from datetime import datetime, timezone
import db

# AccessTokenの長さ
TOKEN_LENGTH = 8

# AccessTokenが有効な秒数
VALID_SECS = 60 * 60

# Return "token" or None
def create_access_token(user_id, conn):
  token = secrets.token_hex(TOKEN_LENGTH)
  now = int(datetime.now(timezone.utc).timestamp())
  sql = (
   "INSERT INTO access_tokens "
   "(token, user_id, created_at) "
   "VALUES (%s, %s, %s)"
  )
  db.insert(conn, sql, (token, user_id, now))
  return token


# Return "user_id" or None
def check_access_token(token, conn):
  now = int(datetime.now(timezone.utc).timestamp())
  oldest_valid = now - (VALID_SECS)
  sql = (
   "SELECT users.id FROM users "
   "INNER JOIN access_tokens ON users.id = access_tokens.user_id "
   "WHERE access_tokens.token = %s "
   " AND access_tokens.force_expired = False "
   " AND %s < access_tokens.created_at"
  )
  return db.query_one(conn, sql, (token, oldest_valid))
