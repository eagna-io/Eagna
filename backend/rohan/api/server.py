from datetime import datetime, timezone
from wsgiref import simple_server
import falcon
import json
from .login import LoginResource
from .user import UserResource

class CORSMiddleware():
  def process_request(self, req, resp):
    resp.set_header('Access-Control-Allow-Origin', '*')

class Server():
  def __init__(self, db, url, port):
    self.db = db
    self.url = url
    self.port = port

  def serve_forever(self):
    app = falcon.API(middleware=[CORSMiddleware()])
    app.add_route("/login", LoginResource(self.db))
    app.add_route("/user", UserResource(self.db))
    httpd = simple_server.make_server(self.url, self.port, app)
    httpd.serve_forever()
