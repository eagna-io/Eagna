from datetime import datetime, timezone
from typing import List, Dict

class Market():
  def __init__(self, *,
                title: str,
                desc: str,
                opening_time: datetime,
                closing_time: datetime,
                outcomes: List[str],
                initial_coin_issue: int):
    self.title = title
    self.desc = desc
    self.opening_time = opening_time
    self.closing_time = closing_time
    self.outcomes = outcomes
    self.initial_coin_issue = initial_coin_issue

  @classmethod
  def init_with_json(cls, json):
    # fromisoformat は末尾のZをパースできない
    opening_time = datetime.fromisoformat(json["opening_time"].replace("Z", "+00:00"))
    closing_time = datetime.fromisoformat(json["closing_time"].replace("Z", "+00:00"))
    return Market(
      title = json["title"],
      desc = json["desc"],
      opening_time = opening_time,
      closing_time = closing_time,
      outcomes = json["outcomes"],
      initial_coin_issue = int(json["coin_info"]["initial_coin_issue"])
    )

  def status(self):
    now = datetime.now(timezone.utc)
    if now < self.opening_time:
      return "Preparing"
    elif now < self.closing_time:
      return "Open"
    else:
      return "Closed"

  def __str__(self):
    return str(self.toDict())

  def toDict(self):
    return {
          "title":                self.title,
          "desc":                 self.desc,
          "opening_time":         str(self.opening_time),
          "closing_time":         str(self.closing_time),
          "outcomes":             self.outcomes,
          "initial_coin_issue":   self.initial_coin_issue,
          "status":               self.status(),
      }
