class Market():
  def __init__(self, *,
                title,
                desc,
                opening_time,
                closing_time,
                outcomes,
                initial_coin_issue):
    self.title = title
    self.desc = desc
    self.opening_time = opening_time
    self.closing_time = closing_time
    self.outcomes = outcomes
    self.initial_coin_issue = initial_coin_issue

  def __str__(self):
    return str(self.toDict())

  def toDict(self):
    return {
          "title":                self.title,
          "desc":                 self.desc,
          "opening_time":         self.opening_time,
          "closing_time":         self.closing_time,
          "outcomes":             self.outcomes,
          "initial_coin_issue":   self.initial_coin_issue
      }


