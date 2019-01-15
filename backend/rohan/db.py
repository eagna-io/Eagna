import os
import psycopg2

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
            return [UserModel(user_id, name, hold_coin)
                for (user_id, name, _, hold_coin)
                in cur.fetchall()]

    def update_user_hold_coins(self, users):
        with self.conn.cursor() as cur:
            for user in users:
                cur.execute('UPDATE users SET hold_coin = %s where id = %s', (user.hold_coin, user.user_id))

class UserModel:
    def __init__(self, user_id, name, hold_coin):
        self.user_id = user_id
        self.name = name
        self.hold_coin = hold_coin
