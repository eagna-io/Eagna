import requests
from datetime import datetime
import json

AdminAccessToken = ""

def main():
    # /users/ リソースのテスト
    test_users_api()

    # /prizes/ リソースのテスト
    test_prizes_api()

    # /markets/ リソースのテスト
    test_markets_api()

    print("[ OK ]")


def test_users_api():
    # signinができることをテストする
    headers = content_type_json(empty_headers())
    payload = {"email": "test-admin@eagna.io", "password": "hogehoge"}
    res = requests.post(url("/users/me/access_token/"),
                        json.dumps(payload),
                        headers=headers)
    assert_eq(res.status_code, 201)
    assert_eq(len(res.json()["token"]), 64)
    global AdminAccessToken
    AdminAccessToken = res.json()["token"]


def test_prizes_api():
    # 初期状態ではPrizeが1つだけ存在していることをテストする
    res = requests.get(url("/prizes/"))
    assert_eq(res.status_code, 200)
    assert_eq(len(res.json()), 1)

    # アクセストークンが正しくても、
    # 不正なペイロードでは賞品が作成できないことをテストする
    headers = content_type_json(bearer_token(empty_headers(),
                                             AdminAccessToken))
    res = requests.post(url("/prizes/"), headers=headers)
    assert_eq(res.status_code, 400)

    # 不正なペイロードでは賞品が作成できないことをテストする
    # nameが空文字列は許可しない
    payload = {
        "name": "",
        "description": "",
        "thumbnailUrl": "",
        "point": 0,
        "available": True,
    }
    res = requests.post(url("/prizes/"), json.dumps(payload), headers=headers)
    assert_eq(res.status_code, 400)

    # 不正なペイロードでは賞品が作成できないことをテストする
    # pointが0は許可しない
    payload = {
        "name": "hoge",
        "description": "",
        "thumbnailUrl": "",
        "point": 0,
        "available": True,
    }
    res = requests.post(url("/prizes/"), json.dumps(payload), headers=headers)
    assert_eq(res.status_code, 400)

    # 不正なペイロードでは賞品が作成できないことをテストする
    # pointが0以下は許可しない
    payload = {
        "name": "hoge",
        "description": "",
        "thumbnailUrl": "",
        "point": -1,
        "available": True,
    }
    res = requests.post(url("/prizes/"), json.dumps(payload), headers=headers)
    assert_eq(res.status_code, 400)

    # 正しいペイロードで賞品が作成できることをテストする
    payload = {
        "name": "Prize Hoge",
        "description": "",
        "thumbnailUrl": "",
        "point": 1,
        "available": True,
    }
    res = requests.post(url("/prizes/"), json.dumps(payload), headers=headers)
    assert_eq(res.status_code, 201)

    # ↑で作成した賞品が取得できるかテストする
    res = requests.get(url("/prizes/"))
    assert_eq(res.status_code, 200)
    assert_eq(len(res.json()), 2)
    prize = next(filter(lambda p: p["name"] == payload["name"], res.json()))
    assert_eq(prize["name"], payload["name"])
    assert_eq(prize["description"], payload["description"])
    assert_eq(prize["thumbnailUrl"], payload["thumbnailUrl"])
    assert_eq(prize["point"], payload["point"])
    assert_eq(prize["available"], payload["available"])


def test_markets_api():
    # 初期状態ではマーケットが存在しないことをテストする
    res = requests.get(url("/markets/"))
    assert_eq(res.status_code, 200)
    assert_eq(len(res.json()), 0)

    # 不正なアクセストークンではマーケットが作成できないことをテストする
    headers = bearer_token(empty_headers(), "invalid_token")
    res = requests.post(url("/markets/"), headers=headers)
    assert_eq(res.status_code, 401)

    # アクセストークンが正しくても、
    # 不正なペイロードではマーケットが作成できないことをテストする
    headers = content_type_json(bearer_token(empty_headers(),
                                             AdminAccessToken))
    res = requests.post(url("/markets/"), headers=headers)
    assert_eq(res.status_code, 400)

    # 不正なペイロードではマーケットが作成できないことをテストする
    # タイトルが空文字列は許可しない
    headers = content_type_json(bearer_token(empty_headers(),
                                             AdminAccessToken))
    invalid_payload = {
        "title": "",
        "description": "hoge",
        "organizerId": "ec2966c5-d661-4a9b-b377-9e00f21d7dd4",
        "lmsrB": 0,
        "open": datetime.utcnow().isoformat() + "Z",
        "close": datetime.utcnow().isoformat() + "Z",
        "totalRewardPoint": 10000,
        "tokens": [],
        "prizes": [],
    }
    res = requests.post(url("/markets/"),
                        json.dumps(invalid_payload),
                        headers=headers)
    assert_eq(res.status_code, 400)

    # 不正なペイロードではマーケットが作成できないことをテストする
    # トークンが空配列は許可しない
    headers = content_type_json(bearer_token(empty_headers(),
                                             AdminAccessToken))
    invalid_payload = {
        "title": "hoge",
        "description": "hoge",
        "organizerId": "ec2966c5-d661-4a9b-b377-9e00f21d7dd4",
        "lmsrB": 0,
        "open": datetime.utcnow().isoformat() + "Z",
        "close": datetime.utcnow().isoformat() + "Z",
        "totalRewardPoint": 10000,
        "tokens": [],
        "prizes": [],
    }
    res = requests.post(url("/markets/"),
                        json.dumps(invalid_payload),
                        headers=headers)
    assert_eq(res.status_code, 400)

    # 不正なペイロードではマーケットが作成できないことをテストする
    # トークン名が空文字列は許可しない
    headers = content_type_json(bearer_token(empty_headers(),
                                             AdminAccessToken))
    invalid_payload = {
        "title": "hoge",
        "description": "",
        "organizerId": "ec2966c5-d661-4a9b-b377-9e00f21d7dd4",
        "lmsrB": 0,
        "open": datetime.utcnow().isoformat() + "Z",
        "close": datetime.utcnow().isoformat() + "Z",
        "totalRewardPoint": 10000,
        "tokens": [{
            "name": "",
            "description": "",
            "thumbnailUrl": ""
        }],
        "prizes": [],
    }
    res = requests.post(url("/markets/"),
                        json.dumps(invalid_payload),
                        headers=headers)
    assert_eq(res.status_code, 400)

    # 不正なペイロードではマーケットが作成できないことをテストする
    # Prizeが空配列は許可しない
    headers = content_type_json(bearer_token(empty_headers(),
                                             AdminAccessToken))
    invalid_payload = {
        "title": "hoge",
        "description": "hoge",
        "organizerId": "ec2966c5-d661-4a9b-b377-9e00f21d7dd4",
        "lmsrB": 0,
        "open": datetime.utcnow().isoformat() + "Z",
        "close": datetime.utcnow().isoformat() + "Z",
        "totalRewardPoint": 10000,
        "tokens": [{
            "name": "hoge",
            "description": "",
            "thumbnailUrl": ""
        }],
        "prizes": [],
    }
    res = requests.post(url("/markets/"),
                        json.dumps(invalid_payload),
                        headers=headers)
    assert_eq(res.status_code, 400)

    # 正しいペイロードでマーケットが作成できることをテストする
    headers = content_type_json(bearer_token(empty_headers(),
                                             AdminAccessToken))
    valid_payload = {
        "title": "hoge",
        "description": "",
        "organizerId": "ec2966c5-d661-4a9b-b377-9e00f21d7dd4",
        "lmsrB": 0,
        "open": datetime.utcnow().isoformat() + "Z",
        "close": datetime.utcnow().isoformat() + "Z",
        "totalRewardPoint": 10000,
        "tokens": [{
            "name": "hoge",
            "description": "",
            "thumbnailUrl": ""
        }],
        "prizes": [{
            "name": "hoge",
            "target": "everyone",
            "thumbnailUrl": ""
        }],
    }
    res = requests.post(url("/markets/"),
                        json.dumps(valid_payload),
                        headers=headers)
    assert_eq(res.status_code, 201)
    assert_eq(type(res.json()["id"]) is str, True)
    market_id = res.json()["id"]

    # ↑で作成したマーケットが取得できることをテストする
    res = requests.get(url(f"/markets/{market_id}/"))
    assert_eq(res.status_code, 200)
    assert_eq(res.json()["id"], market_id)
    assert_eq(res.json()["title"], valid_payload["title"])
    assert_eq(res.json()["description"], valid_payload["description"])
    assert_eq(res.json()["organizerId"], valid_payload["organizerId"])
    assert_eq(res.json()["lmsrB"], valid_payload["lmsrB"])
    assert_eq(res.json()["open"], valid_payload["open"])
    assert_eq(res.json()["close"], valid_payload["close"])
    assert_eq(res.json()["totalRewardPoint"],
              valid_payload["totalRewardPoint"])
    assert_eq(res.json()["tokens"], valid_payload["tokens"])
    assert_eq(len(res.json()["prizes"]), 1)
    assert_eq(res.json()["prizes"][0]["name"],
              valid_payload["prizes"][0]["name"])
    assert_eq(res.json()["prizes"][0]["target"],
              valid_payload["prizes"][0]["target"])
    assert_eq(
        res.json()["prizes"][0]["thumbnailUrl"],
        valid_payload["prizes"][0]["thumbnailUrl"],
    )


## Utils
def url(path):
    return f"http://localhost:8081{path}"


def empty_headers():
    return {}


def content_type_json(headers):
    headers["Content-Type"] = "application/json"
    return headers


def bearer_token(headers, token):
    headers["Authorization"] = f"Bearer: {token}"
    return headers


def assert_eq(found, expected):
    assert found == expected, f"expected [{expected}] but found [{found}]"


main()
