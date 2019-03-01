import json
import os
from multiprocessing import Process

from market_observer import observe_market
from api.server import Server

# Observerの起動
p = Process(target = observe_market)
p.start()
print("Start to observe market open/close")

# API server の起動
bind_host = os.getenv('BIND_HOST', '127.0.0.1')
bind_port = os.getenv('BIND_PORT', 8000)
print(f"Server start on {bind_host}:{bind_port}")
server = Server(bind_host, bind_port)
server.serve_forever()
