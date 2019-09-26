import requests
from datetime import datetime
import json

AdminAccessToken = "test_admin_access_token"
UserAccessToken = "test_user_access_token"

def main():
    # /users/ リソースのテスト
    test_users_api()

    # /markets/ リソースのテスト
    test_markets_api()

    print("[ OK ]")


def test_users_api():
    # ユーザーがまだ作られていないことをテストする
    # Firebaseには登録済みで、AccessTokenは取得している
    headers = bearer_token({}, UserAccessToken)
    res = requests.get(url("/users/me/"), headers=headers)
    assert_eq(res.status_code, 401)

    # 不正なペイロードではユーザーが作成できないことをテストする
    # 名前が空文字列は許可しない
    payload = {
        "name": "",
        "email": "hoge@eagna.io",
    }
    headers = content_type_json(bearer_token({}, UserAccessToken))
    res = requests.post(url("/users/"), json.dumps(payload), headers=headers,)
    assert_eq(res.status_code, 400)

    # 不正なペイロードではユーザーが作成できないことをテストする
    # メアドが空文字列は許可しない
    payload = {
        "name": "Hoge Hogeo",
        "email": "",
    }
    headers = content_type_json(bearer_token({}, UserAccessToken))
    res = requests.post(url("/users/"), json.dumps(payload), headers=headers,)
    assert_eq(res.status_code, 400)

    # 正しいペイロードでユーザーが作成できることをテストする
    payload = {
        "name": "Hoge Hogeo",
        "email": "hoge@eagna.io",
    }
    headers = content_type_json(bearer_token({}, UserAccessToken))
    res = requests.post(url("/users/"), json.dumps(payload), headers=headers,)
    assert_eq(res.status_code, 201)

    # ↑で作成したユーザーが取得できることをテストする
    headers = bearer_token({}, UserAccessToken)
    res = requests.get(url("/users/me/"), headers=headers)
    assert_eq(res.status_code, 200)
    assert_eq(res.json()["name"], "Hoge Hogeo")
    assert_eq(res.json()["email"], "hoge@eagna.io")
    assert_eq(res.json()["isAdmin"], False)
    assert_eq(res.json()["point"], 0)


def test_markets_api():
    # 初期状態ではマーケットが存在しないことをテストする
    res = requests.get(url("/markets/"))
    assert_eq(res.status_code, 200)
    assert_eq(len(res.json()), 0)

    # 不正なアクセストークンではマーケットが作成できないことをテストする
    invalid_token = "hoge"
    headers = bearer_token(empty_headers(), invalid_token)
    res = requests.post(url("/markets/"), headers=headers)
    assert_eq(res.status_code, 401)

    # アクセストークンが正しくても、
    # 不正なペイロードではマーケットが作成できないことをテストする
    headers = content_type_json(bearer_token(empty_headers(), AdminAccessToken))
    res = requests.post(url("/markets/"), headers=headers)
    assert_eq(res.status_code, 400)

    # 不正なペイロードではマーケットが作成できないことをテストする
    # タイトルが空文字列は許可しない
    headers = content_type_json(bearer_token(empty_headers(), AdminAccessToken))
    invalid_payload = {
        'title': "",
        'description': "hoge",
        'organizerId': "ec2966c5-d661-4a9b-b377-9e00f21d7dd4",
        'lmsrB': 0,
        'open': datetime.utcnow().isoformat() + 'Z',
        'close': datetime.utcnow().isoformat() + 'Z',
        'totalRewardPoint': 10000,
        'tokens': [],
        'prizes': [],
    }
    res = requests.post(url("/markets/"), json.dumps(invalid_payload), headers=headers)
    assert_eq(res.status_code, 400)

    # 不正なペイロードではマーケットが作成できないことをテストする
    # トークンが空配列は許可しない
    headers = content_type_json(bearer_token(empty_headers(), AdminAccessToken))
    invalid_payload = {
        'title': "hoge",
        'description': "hoge",
        'organizerId': "ec2966c5-d661-4a9b-b377-9e00f21d7dd4",
        'lmsrB': 0,
        'open': datetime.utcnow().isoformat() + 'Z',
        'close': datetime.utcnow().isoformat() + 'Z',
        'totalRewardPoint': 10000,
        'tokens': [],
        'prizes': [],
    }
    res = requests.post(url("/markets/"), json.dumps(invalid_payload), headers=headers)
    assert_eq(res.status_code, 400)

    # 不正なペイロードではマーケットが作成できないことをテストする
    # トークン名が空文字列は許可しない
    headers = content_type_json(bearer_token(empty_headers(), AdminAccessToken))
    invalid_payload = {
        'title': "hoge",
        'description': "",
        'organizerId': "ec2966c5-d661-4a9b-b377-9e00f21d7dd4",
        'lmsrB': 0,
        'open': datetime.utcnow().isoformat() + 'Z',
        'close': datetime.utcnow().isoformat() + 'Z',
        'totalRewardPoint': 10000,
        'tokens': [{"name": "", "description": "", "thumbnailUrl": ""}],
        'prizes': [],
    }
    res = requests.post(url("/markets/"), json.dumps(invalid_payload), headers=headers)
    assert_eq(res.status_code, 400)

    # 正しいペイロードでマーケットが作成できることをテストする
    headers = content_type_json(bearer_token(empty_headers(), AdminAccessToken))
    valid_payload = {
        'title': "hoge",
        'description': "",
        'organizerId': "ec2966c5-d661-4a9b-b377-9e00f21d7dd4",
        'lmsrB': 0,
        'open': datetime.utcnow().isoformat() + 'Z',
        'close': datetime.utcnow().isoformat() + 'Z',
        'totalRewardPoint': 10000,
        'tokens': [{"name": "hoge", "description": "", "thumbnailUrl": ""}],
        'prizes': [],
    }
    res = requests.post(url("/markets/"), json.dumps(valid_payload), headers=headers)
    assert_eq(res.status_code, 201)
    market_id = res.json()

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
    assert_eq(res.json()["totalRewardPoint"], valid_payload["totalRewardPoint"])
    assert_eq(res.json()["tokens"], valid_payload["tokens"])
    assert_eq(res.json()["prizes"], valid_payload["prizes"])


## Utils
def url(path):
    return f"http://localhost:8081{path}"

def empty_headers():
    return {}

def content_type_json(headers):
    headers['Content-Type'] = 'application/json'
    return headers

def bearer_token(headers, token):
    headers['Authorization'] = f"Bearer: {token}"
    return headers

def assert_eq(found, expected):
    assert found == expected, f"expected [{expected}] but found [{found}]"

main()
