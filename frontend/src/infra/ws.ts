import * as D from "@mojotech/json-type-validation";

export const WS_URL = process.env.REACT_APP_API_BASE + "/ws";

export interface Params {
  marketId: string;
  onFeedMsg: (msg: FeedMsg) => void;
}

export const open = ({ marketId, onFeedMsg }: Params) => {
  const ws = new WebSocket(`${WS_URL}/${marketId}`);
  ws.onmessage = event => {
    const data = JSON.parse(event.data);
    const decoded = FeedMsgDecoder.run(data);
    if (decoded.ok) {
      onFeedMsg(decoded.result);
    } else {
      console.error(decoded);
    }
  };
};

export interface FeedMsg {
  outcome: "realize" | "unrealize";
  accountName: string;
  timestamp: number;
}

const FeedMsgDecoder: D.Decoder<FeedMsg> = D.object({
  outcome: D.union(
    D.constant<"realize">("realize"),
    D.constant<"unrealize">("unrealize")
  ),
  accountName: D.string(),
  timestamp: D.number()
});
