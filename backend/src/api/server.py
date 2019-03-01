from datetime import datetime, timezone
from wsgiref import simple_server
import falcon
import json
from .login import LoginResource
from .me import MeResource
from .market import MarketResource
from .order import OrderResource

class CORSMiddleware():
  def process_request(self, req, resp):
    resp.set_header('Access-Control-Allow-Origin', '*')

class Server():
  def __init__(self, url, port):
    self.url = url
    self.port = port

  def serve_forever(self):
    app = falcon.API(middleware=[CORSMiddleware()])
    app.add_route("/login", LoginResource())
    app.add_route("/me", MeResource())
    app.add_route("/market/{id}", MarketResource())
    app.add_route("/order", OrderResource())
    httpd = simple_server.make_server(self.url, self.port, app)
    httpd.serve_forever()
