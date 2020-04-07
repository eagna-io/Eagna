import * as D from "@mojotech/json-type-validation";

export const WS_URL = process.env.REACT_APP_WS_API_BASE;

export interface Params<T> {
  onMsg: (msg: T) => void;
  msgDecoder: D.Decoder<T>;
}

export const open = <T>({ onMsg, msgDecoder }: Params<T>): WebSocket => {
  const ws = new WebSocket(`${WS_URL}`);
  ws.onmessage = event => {
    const data = JSON.parse(event.data);
    const decoded = msgDecoder.run(data);
    if (decoded.ok) {
      onMsg(decoded.result);
    } else {
      console.error(decoded);
    }
  };
  return ws;
};
