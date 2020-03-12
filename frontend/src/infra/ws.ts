import * as D from "@mojotech/json-type-validation";
import moment, { Moment } from "moment";

export const WS_URL = process.env.REACT_APP_API_BASE + "/ws";

export interface Params {
  marketId: string;
  onOrderMsg: (msg: OrderMsg) => void;
}

export const open = ({ marketId, onOrderMsg }: Params) => {
  const ws = new WebSocket(`${WS_URL}/${marketId}`);
  ws.onmessage = event => {
    const data = JSON.parse(event.data);
    const decoded = OrderMsgDecoder.run(data);
    if (decoded.ok) {
      onOrderMsg(decoded.result);
    } else {
      console.error(decoded);
    }
  };
};

export interface OrderMsg {
  type: "order";
  outcome: "realize" | "unrealize";
  accountName: string;
  time: Moment;
  tipCost: number;
}

const OrderMsgDecoder: D.Decoder<OrderMsg> = D.object({
  type: D.constant<"order">("order"),
  outcome: D.union(
    D.constant<"realize">("realize"),
    D.constant<"unrealize">("unrealize")
  ),
  accountName: D.string(),
  time: D.string().map(s => moment(s)),
  tipCost: D.number()
});
