from math import log, exp, floor

def cost(tokens, b):
  return normalize(b * log(sum([exp(q/b) for q in tokens])))

def normalize(n):
  return floor(1000 * n)
