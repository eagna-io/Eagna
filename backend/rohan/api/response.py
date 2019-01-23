import json

def success(res):
  return response(True, res)

def failure(res):
  return response(False, res)

def response(success, res):
  return json.dumps({
    "success": success,
    "result": res,
  })
