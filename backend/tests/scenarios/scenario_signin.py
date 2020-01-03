import requests
from datetime import datetime
import json

AdminAccessToken = ""


def main():
    signin()
    signout()


def signin():
    # 不正なパスワードではsigninができないことをテストする
    headers = content_type_json(empty_headers())
    payload = {"email": "test-admin@eagna.io", "password": "invalid"}
    res = requests.post(url("/users/me/access_token/"),
                        json.dumps(payload),
                        headers=headers)
    assert_eq(res.status_code, 401)
    assert "token" not in res.json()

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


def signout():
    # 取得済みのトークンを無効化
    headers = bearer_token({}, AdminAccessToken)
    res = requests.delete(url("/users/me/access_token/"), headers=headers)
    assert_eq(res.status_code, 204)

    # ちゃんと無効化されていることをテスト
    headers = bearer_token({}, AdminAccessToken)
    res = requests.get(url("/users/me/"), headers=headers)
    assert_eq(res.status_code, 401)


## Utils
def url(path):
    return f"http://localhost:8081{path}"


def empty_headers():
    return {}


def content_type_json(headers):
    headers["Content-Type"] = "application/json"
    return headers


def bearer_token(headers, token):
    headers["Authorization"] = f"Bearer {token}"
    return headers


def assert_eq(found, expected):
    assert found == expected, f"expected [{expected}] but found [{found}]"


main()
