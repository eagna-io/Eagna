from . import response

class MarketResource():
  def __init__(self, market):
    self.market = market

  def on_get(self, req, resp):
    resp.body = response.success(self.market.toDict())
