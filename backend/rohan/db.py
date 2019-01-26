import os
import secrets
import psycopg2
from datetime import datetime, timezone

class DB:
  conn = None

  def __init__(self, conn):
    self.conn = conn

  @classmethod
  def init_with_env(cls):
    uri = os.environ.get('DATABASE_URL')
    return DB.init_with_uri(uri)

  @classmethod
  def init_with_uri(cls, uri):
    return DB(psycopg2.connect(uri))

  def commit(self):
    self.conn.commit()

  def query_users(self):
    with self.conn.cursor() as cur:
      cur.execute('SELECT * FROM users')
      return [UserModel(name, coins)
      for (name, _, coins)
        in cur.fetchall()]

  def update_user_coins(self, user):
    with self.conn.cursor() as cur:
      cur.execute('UPDATE users SET coins = %s where name = %s', (user.coins, user.name))

  def check_login(self, user, hashedPass):
    with self.conn.cursor() as cur:
      cur.execute('SELECT name FROM users WHERE name = %s AND hashed_pass = %s', (user, hashedPass))
      name = cur.fetchone()
      if name == None:
        return False
      else:
        return True

  def create_access_token(self, name, now):
    token = secrets.token_hex(8)
    with self.conn.cursor() as cur:
      cur.execute('INSERT INTO access_tokens (token, user_name, is_valid, created_at) VALUES (%s, %s, %s, %s)', (token, name, True, now))
      return token

  def check_access_token(self, token):
    now = int(datetime.now(timezone.utc).timestamp())
    expired_time = now - (60 * 60)
    with self.conn.cursor() as cur:
      cur.execute('SELECT users.name, users.coins FROM users INNER JOIN access_tokens ON users.name = access_tokens.user_name WHERE access_tokens.token = %s AND access_tokens.is_valid = True AND access_tokens.created_at > %s', (token, expired_time))
      res = cur.fetchone()
      if res == None:
        return None
      else:
        (name, coins) = res
        return UserModel(name, coins)

class UserModel:
    def __init__(self, name, coins):
        self.name = name
        self.coins = coins
