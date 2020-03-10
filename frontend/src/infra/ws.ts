export const WS_URL = process.env.REACT_APP_API_BASE + "/ws";

export interface Params {
  marketId: string;
  onFeedMsg: (msg: FeedMsg) => void;
}

export const open = ({ marketId, onFeedMsg }: Params) => {
  const ws = new WebSocket(`${WS_URL}/${marketId}`);
  ws.onmessage = event => {
    const data = JSON.parse(event.data);
    console.log(event);

    // TODO
    onFeedMsg(data as FeedMsg);
  };
};

export interface FeedMsg {
  outcomeId: string;
  accountId: string;
  timestamp: number;
}
