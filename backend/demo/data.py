from datetime import datetime, timezone, timedelta
import psycopg2

from market import insert_market_data
from user import insert_sample_user_data

def feed_user_data(conn):
  insert_sample_user_data(conn)

def feed_market_data(conn):
    return insert_market_data(conn, demo_market())


def demo_market():
  return {
    "title": "2019シーズンのF1チャンピオンは？",
    "organizer": "RohanMarket.inc",
    "short_desc": "2019シーズンのF1年間チャンピオンを予想する",
    "desc": demo_market_desc(),
    "open_time": now() - timedelta(minutes=1), # 1分前にopen
    "close_time": now() + timedelta(hours=1), # 1時間後にclose
    "initial_coin_issue": 10000,
    "status": "preparing",
    "tokens": demo_tokens(),
  }

def demo_market_desc():
  return (
    "2019シーズンのF1選手権年間チャンピオンを予想する。\n"
    "選択可能な選手は以下。\n"
    "- ルイス・ハミルトン\n"
    "- バルテリ・ボッタス\n"
    "- セバスチャン・ベッテル\n"
    "- シャルル・ルクレール\n"
    "\n"
    "enjoy.\n"
  )

def demo_tokens():
  return [
    {
      "name": "Lewis",
      "desc": "Lewis Carl Davidson Hamilton",
      "initial_price": 50,
    },
    {
      "name": "Valtteri",
      "desc": "Valtteri Viktor Bottas",
      "initial_price": 20,
    },
    {
      "name": "Sebastian",
      "desc": "Sebastian Vettel",
      "initial_price": 40,
    },
    {
      "name": "Charles",
      "desc": "Charles Leclerc",
      "initial_price": 20,
    },
  ]


def now():
  return datetime.now(timezone.utc)
