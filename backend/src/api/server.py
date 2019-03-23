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
  def __init__(self, url, port):
    self.url = url
    self.port = port

  def serve_forever(self):
    app = falcon.API(middleware=[CORSMiddleware()])
    app.add_route("/accesstoken/{access_token}", AccessTokenResource())
    app.add_route("/accesstoken", AccessTokenResource())
    app.add_route("/me", MeResource())
    app.add_route("/markets/{id}", MarketResource())
    app.add_route("/order", OrderResource())
    httpd = simple_server.make_server(self.url, self.port, app)
    httpd.serve_forever()
