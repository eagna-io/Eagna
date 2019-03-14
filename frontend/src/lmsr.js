const B = 1;
const MINI = 1000;

export function prices(distributions) {
  const denom = distributions.reduce((acc, cur) => acc + Math.exp(cur/B), 0);
  return distributions.map(n => normalize(Math.exp(n/B) / denom))
}

export function cost(distributions) {
  const res = B * Math.log(distributions.reduce((acc, cur) => acc + Math.exp(cur/B), 0));
  return normalize(res);
}

export function normalize(n) {
  return Math.floor( n * MINI )
}
