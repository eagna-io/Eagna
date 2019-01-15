from unittest import TestCase
from rohan.db import DB

class TestDB(TestCase):
    db = DB.init_with_env()

    def test_query_users(self):
        res = self.db.query_users()
        self.assertEqual(len(res), 2)

    def test_update_user_hold_coins(self):
        users = self.db.query_users()

        users[0].hold_coin = 50
        users[1].hold_coin = 100
        self.db.update_user_hold_coins(users)

        res = self.db.query_users()
        self.assertEqual(res[0].hold_coin, 50)
        self.assertEqual(res[1].hold_coin, 100)
