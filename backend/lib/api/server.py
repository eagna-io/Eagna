from datetime import datetime, timezone
from wsgiref import simple_server
import falcon
import json
from .accesstoken import AccessTokenResource
from .me import MeResource
from .market import MarketResource
from .order import OrderResource

class CORSMiddleware():
  def process_request(self, req, resp):
    resp.set_header('Access-Control-Allow-Origin', '*')
    resp.set_header('Access-Control-Allow-Headers', 'Content-Type')

class Server():
  def __init__(self, url, port, db_url):
    self.url = url
    self.port = port
    self.db_url = db_url

  def serve_forever(self):
    app = falcon.API(middleware=[CORSMiddleware()])
    app.add_route("/accesstoken/{access_token}", AccessTokenResource(self.db_url))
    app.add_route("/accesstoken", AccessTokenResource(self.db_url))
    app.add_route("/me", MeResource(self.db_url))
    app.add_route("/markets/{id}", MarketResource(self.db_url))
    app.add_route("/order", OrderResource(self.db_url))
    httpd = simple_server.make_server(self.url, self.port, app)
    httpd.serve_forever()
