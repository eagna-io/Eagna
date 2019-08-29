import os
import sys
import psycopg2
import numpy as np
import requests

EAGNA_PG_URL = os.environ["EAGNA_PG_URL"]

MAILGUN_API_URL = "https://api.mailgun.net/v3/{domain}/messages".format(domain = 'eagna.io')
MAILGUN_API_KEY = os.environ["MAILGUN_API_KEY"]

def main():
    market_id = sys.argv[1]
    gift_link_list = sys.argv[2].split(",")

    users = query_user_coins(market_id)
    lucky_user_list = choice_reward_users(users, len(gift_link_list))
    market_title = query_market_title(market_id)

    for (coin, name, email), gift_link in zip(lucky_user_list, gift_link_list):
        send_raward_mail(market_title, gift_link, email)



exclude_user_emails = [
    "takatomgoo@gmail.com",
    "fullyou0798@gmail.com",
    "pinkgreen0304@gmail.com",
    "marketrohan@gmail.com",
]
## DBから各ユーザーの獲得コイン量とユーザーの情報を取得する
## 
## Returns
## [(coin: number, name: string, email: string)]
def query_user_coins(market_id):
    cur = psycopg2.connect(EAGNA_PG_URL).cursor()
    cur.execute("SELECT SUM(amount_coin), name, email FROM orders INNER JOIN users ON orders.user_id = users.fb_uid WHERE market_id = '{market_id}' GROUP BY name, email".format(market_id = market_id))
    return [
        (coin, name, email)
        for (coin, name, email)
        in cur.fetchall()
        if email not in exclude_user_emails
    ]

def query_market_title(market_id):
    cur = psycopg2.connect(EAGNA_PG_URL).cursor()
    cur.execute("SELECT title FROM markets WHERE id = '{market_id}'".format(market_id = market_id))
    return cur.fetchone()[0]

## 獲得コイン量に応じて当選者を抽出する
def choice_reward_users(users, n):
    coin_list = np.asarray([coin for (coin, name, email) in users])
    prob_list = coin_list / sum(coin_list)
    chosen_list = np.random.choice(len(coin_list), n, p=prob_list)
    return [users[i] for i in chosen_list]


## 当選メールを送る
def send_raward_mail(market_title, gift_link, user_addr):
    FROM = "Eagna 運営 <info@eagna.io>"
    SUBJECT = "[Eagna] ギフトを贈ります！"
    text = """
Eagnaへのご参加ありがとうございました！

以下のマーケットで予測報酬が当選しましたのでお送り致します。

「{market}」

今後もEagnaを宜しくお願い致します！

◆こちらのリンクからギフトを受け取ることができます
{gift_link}
    """.format(market = market_title, gift_link = gift_link).strip()

    data = {
        'from': FROM,
        'to': user_addr,
        'subject': SUBJECT,
        'text': text,
    }

    r = requests.post(MAILGUN_API_URL, auth = ('api', MAILGUN_API_KEY), data = data)

main()
