from math import log, exp, floor

B = 1

def cost(tokens):
  return normalize(B * log(sum([exp(q/B) for q in tokens])))

def normalize(n):
  return floor(1000 * n)
