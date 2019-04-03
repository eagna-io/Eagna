export function timestampToStr(ts) {
  const tokyo_ts = ts + (60 * 60 * 9);
  const date = new Date(tokyo_ts * 1000);
  const year = date.getFullYear();
  const month = date.getMonth() + 1;
  const day = date.getDate();
  const hour = ('0' + date.getHours()).slice(-2);
  const min = ('0' + date.getMinutes()).slice(-2);
  const sec = ('0' + date.getSeconds()).slice(-2);
  return `${year}-${month}-${day} ${hour}:${min}:${sec}`;
}
