import requests
from datetime import datetime, timedelta
import json
import time

AdminAccessToken = ""


def main():
    signin()

    prize = create_prize()

    market = create_market()

    res_open_market = check_markets()
    assert_eq(len(res_open_market["openMarkets"]), 1)

    join_market(market["id"])

    time.sleep(5)

    res_close_market = check_markets()
    assert_eq(len(res_close_market["closeMarkets"]), 1)

    resolve_market(market["id"])

    my_info = get_my_info()
    print(my_info)

    request_prize_trade(prize["id"])

    new_my_info = get_my_info()
    print(new_my_info)
    assert_eq(len(new_my_info["prizeTradeHistory"]), 1)
    assert_eq(my_info["point"] - new_my_info["point"], prize["point"])

    print("[ OK ]")


def signin():
    headers = content_type_json({})
    payload = {
        "email": "test-admin@eagna.io",
        "password": "hogehoge"
    }
    res = requests.post(url("/users/me/access_token/"), json.dumps(payload), headers=headers)
    assert_eq(res.status_code, 201)
    global AdminAccessToken
    AdminAccessToken = res.json()["token"]


# 報酬を作成する
def create_prize():
    headers = content_type_json(bearer_token(empty_headers(),
                                             AdminAccessToken))
    payload = {
        "name": "The prize",
        "description": "",
        "thumbnailUrl": "",
        "point": 100,
        "available": True,
    }
    res = requests.post(url("/prizes/"), json.dumps(payload), headers=headers)
    assert_eq(res.status_code, 201)
    return res.json()


# マーケットを作成する
def create_market():
    headers = content_type_json(bearer_token(empty_headers(),
                                             AdminAccessToken))
    payload = {
        'title': "The market",
        'description': "",
        'organizerId': "ec2966c5-d661-4a9b-b377-9e00f21d7dd4",
        'lmsrB': 100,
        'open': datetime.utcnow().isoformat() + 'Z',
        'close': (datetime.utcnow() + timedelta(seconds=5)).isoformat() + 'Z',
        'totalRewardPoint': 10000,
        'tokens': [{
            "name": "hoge",
            "description": "",
            "thumbnailUrl": ""
        }],
        'prizes': [{
            "name": "hoge",
            "target": "everyone",
            "thumbnailUrl": ""
        }],
    }
    res = requests.post(url("/markets/"), json.dumps(payload), headers=headers)
    assert_eq(res.status_code, 201)
    return res.json()


def check_markets():
    headers = {"X-Appengine-Cron": "True"}
    res = requests.get(url("/cronjob/check_markets/"), headers=headers)
    assert_eq(res.status_code, 200)
    return res.json()


def join_market(market_id):
    headers = content_type_json(bearer_token(empty_headers(), AdminAccessToken))
    payload = {
        "amountToken": 0,
        "amountCoin": 0,
        "time": datetime.utcnow().isoformat() + 'Z',
        "type": "coinSupply",
    }
    res = requests.post(url(f"/markets/{market_id}/orders/"),
                        json.dumps(payload),
                        headers=headers)
    assert_eq(res.status_code, 201)


def resolve_market(market_id):
    headers = content_type_json(bearer_token(empty_headers(),
                                             AdminAccessToken))
    payload = {
        "status": "resolved",
        "resolvedTokenName": "hoge",
    }
    res = requests.put(url(f"/markets/{market_id}/"),
                       json.dumps(payload),
                       headers=headers)
    assert_eq(res.status_code, 201)


def get_my_info():
    headers = content_type_json(bearer_token(empty_headers(), AdminAccessToken))
    res = requests.get(url("/users/me/"), headers=headers)
    assert_eq(res.status_code, 200)
    return res.json()


def request_prize_trade(prize_id):
    headers = content_type_json(bearer_token(empty_headers(), AdminAccessToken))
    payload = {
        "prizeId": prize_id,
    }
    res = requests.post(url("/users/me/prize_trade_history/"),
                        json.dumps(payload),
                        headers=headers)
    assert_eq(res.status_code, 201)


## Utils
def url(path):
    return f"http://localhost:8081{path}"


def empty_headers():
    return {}


def content_type_json(headers):
    headers['Content-Type'] = 'application/json'
    return headers


def bearer_token(headers, token):
    headers['Authorization'] = f"Bearer {token}"
    return headers


def assert_eq(found, expected):
    assert found == expected, f"expected [{expected}] but found [{found}]"


main()
