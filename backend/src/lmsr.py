from math import log, exp

def cost(tokens, b):
  return b * log(sum([exp(q/b) for q in tokens]))
