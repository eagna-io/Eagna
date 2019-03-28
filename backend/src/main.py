import json
import os
from multiprocessing import Process

from market_observer import observe_market
from api.server import Server

# Observerの起動
p = Process(target = observe_market)
p.start()
print("Start to observe market open/close")

# 環境変数の取得
# 環境変数が指定されていない場合はエラー
bind_host = os.environ['ROHAN_BIND_HOST']
bind_port = os.environ['ROHAN_BIND_PORT']

# API server の起動
print(f"Server start on {bind_host}:{bind_port}")
server = Server(bind_host, bind_port)
server.serve_forever()
