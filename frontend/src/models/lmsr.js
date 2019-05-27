const MINI = 1000;

export function prices(lmsrB, distributions) {
  const denom = distributions.reduce((acc, cur) => acc + Math.exp(cur/lmsrB), 0);
  return distributions.map(n => normalize(Math.exp(n/lmsrB) / denom))
}

export function cost(lmsrB, distributions) {
  const res = lmsrB * Math.log(distributions.reduce((acc, cur) => acc + Math.exp(cur/lmsrB), 0));
  return normalize(res);
}

export function normalize(n) {
  return Math.floor( n * MINI )
}
