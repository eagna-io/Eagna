import requests
from datetime import datetime
import json

AdminAccessToken = "test_admin_access_token"
UserAccessToken = "test_user_access_token"

def main():
    get_user_test_1()
    create_user_test()
    get_user_test_2()

    get_markets_test_1()
    create_market_test()

    print("[ OK ]")


# ユーザーがまだ作られていないことをテストする
# Firebaseには登録済みで、AccessTokenは取得している
def get_user_test_1():
    headers = bearer_token({}, UserAccessToken)
    res = requests.get(url("/users/me/"), headers=headers)
    assert_eq(res.status_code, 401)

def create_user_test():
    # valid payload
    payload = {
        "name": "Hoge Hogeo",
        "email": "hoge@eagna.io",
    }
    headers = content_type_json(bearer_token({}, UserAccessToken))
    res = requests.post(url("/users/"), json.dumps(payload), headers=headers,)
    assert_eq(res.status_code, 201)

def get_user_test_2():
    headers = bearer_token({}, UserAccessToken)
    res = requests.get(url("/users/me/"), headers=headers)
    assert_eq(res.status_code, 200)
    assert_eq(res.json()["name"], "Hoge Hogeo")
    assert_eq(res.json()["email"], "hoge@eagna.io")
    assert_eq(res.json()["isAdmin"], False)
    assert_eq(res.json()["point"], 0)


def get_markets_test_1():
    res = requests.get(url("/markets/"))
    assert_eq(res.status_code, 200)
    assert_eq(len(res.json()), 0)

def create_market_test():
    # Test invalid access token
    invalid_token = "hoge"
    headers = bearer_token(empty_headers(), invalid_token)
    res = requests.post(url("/markets/"), headers=headers)
    assert_eq(res.status_code, 401)

    # Test valid access token but invalid payload
    headers = content_type_json(bearer_token(empty_headers(), AdminAccessToken))
    res = requests.post(url("/markets/"), headers=headers)
    assert_eq(res.status_code, 400)

    # Valid request
    # でも↓ははじくように修正する必要ある
    headers = content_type_json(bearer_token(empty_headers(), AdminAccessToken))
    invalid_payload = {
        'title': "",
        'description': "",
        'organizerId': "ec2966c5-d661-4a9b-b377-9e00f21d7dd4",
        'lmsrB': 0,
        'open': datetime.utcnow().isoformat() + 'Z',
        'close': datetime.utcnow().isoformat() + 'Z',
        'totalRewardPoint': 10000,
        'tokens': [],
        'prizes': [],
    }
    res = requests.post(url("/markets/"), json.dumps(invalid_payload), headers=headers)
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
    headers['Authorization'] = f"Bearer: {token}"
    return headers

def assert_eq(found, expected):
    assert found == expected, f"expected [{expected}] but found [{found}]"

main()
