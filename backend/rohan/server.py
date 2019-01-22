from datetime import datetime, timezone
from wsgiref import simple_server
import falcon
import json

class LoginResource():
  def __init__(self, db):
    self.db = db

  def on_get(self, req, resp):
    user_name = req.params.get("user")
    hashed_pass = req.params.get("pass")
    if user_name == None or hashed_pass == None:
      resp.body = json.dumps(failure("parameter is not enough"))
      return

    if self.db.check_login(user_name, hashed_pass) == False:
      resp.body = json.dumps(failure("invalid user or password"))
      return

    now = int(datetime.now(timezone.utc).timestamp())
    access_token = self.db.create_access_token(user_name, now)

    resp.body = json.dumps(success(access_token))
    return


def success(result):
  return {
    "success": True,
    "result": result,
  }

def failure(result):
  return {
    "success": False,
    "result": result,
  }

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
    httpd = simple_server.make_server(self.url, self.port, app)
    httpd.serve_forever()
