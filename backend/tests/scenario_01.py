import requests
from datetime import datetime
import json

AdminAccessToken = "test_admin_access_token"

def main():
    get_markets_test_1()
    create_market_test()

    print("[ OK ]")


def get_markets_test_1():
    res = requests.get(url("/markets/"))
    assert res.status_code == 200, "/markets/ へのリクエストに失敗"
    assert len(res.json()) == 0, "/markets/ はまだ作成していないはず"

def create_market_test():
    # Test invalid access token
    invalid_token = "hoge"
    headers = bearer_token(empty_headers(), invalid_token)
    res = requests.post(url("/markets/"), headers=headers)
    assert res.status_code == 401, f"found status code {res.status_code}"

    # Test valid access token but invalid payload
    headers = content_type_json(bearer_token(empty_headers(), AdminAccessToken))
    res = requests.post(url("/markets/"), headers=headers)
    assert res.status_code == 400, f"found status code {res.status_code}"

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
    print(json.dumps(invalid_payload))
    res = requests.post(url("/markets/"), json.dumps(invalid_payload), headers=headers)
    assert res.status_code == 201, f"found status code {res.status_code}"

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

main()
