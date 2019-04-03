from datetime import datetime, timezone, timedelta

def demo_users():
  return [
    {
      "name": "alice",
      "email": "alice@rohanmarket.com",
      "raw_pass": "alice"
    },
    {
      "name": "bob",
      "email": "bob@rohanmarket.com",
      "raw_pass": "bob"
    }
  ]

def demo_market():
  return {
    "title": "2019シーズンのF1チャンピオンは？",
    "organizer": "RohanMarket.inc",
    "shortDesc": "2019シーズンのF1年間チャンピオンを予想する",
    "desc": demo_market_desc(),
    "openTime": now() - timedelta(minutes=1), # 1分前にopen
    "closeTime": now() + timedelta(hours=1), # 1時間後にclose
    "startCoinSupply": 10000,
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
      "initialPrice": 50,
    },
    {
      "name": "Valtteri",
      "desc": "Valtteri Viktor Bottas",
      "initialPrice": 20,
    },
    {
      "name": "Sebastian",
      "desc": "Sebastian Vettel",
      "initialPrice": 40,
    },
    {
      "name": "Charles",
      "desc": "Charles Leclerc",
      "initialPrice": 20,
    },
  ]


def now():
  return datetime.now(timezone.utc)
