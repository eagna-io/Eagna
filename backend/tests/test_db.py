from unittest import TestCase
from rohan.db import DB

class TestDB(TestCase):

  def test_query_users(self):
    db = DB.init_with_env()
    res = db.query_users()
    self.assertEqual(len(res), 2)

  def test_update_user_hold_coins(self):
    db = DB.init_with_env()
    users = db.query_users()

    users[0].hold_coin = 50
    users[1].hold_coin = 100
    db.update_user_hold_coins(users)

    res = db.query_users()
    self.assertEqual(res[0].hold_coin, 50)
    self.assertEqual(res[1].hold_coin, 100)

  def test_check_login(self):
    db = DB.init_with_env()
    res = db.check_login('alice', '2bd806c97f0e00af1a1fc3328fa763a9269723c8db8fac4f93af71db186d6e90')
    self.assertEqual(res, True)

    res2 = db.check_login('bob', '2bd806c97f0e00af1a1fc3328fa763a9269723c8db8fac4f93af71db186d6e90')
    self.assertEqual(res2, False)

  def test_create_access_token(self):
    db = DB.init_with_env()
    token = db.create_access_token("alice", 1548178960)
    self.assertEqual(len(token), 16)
