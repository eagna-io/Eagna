import * as D from "@mojotech/json-type-validation";
import moment, { Moment } from "moment";

export const WS_URL = process.env.REACT_APP_WS_API_BASE;

export interface Params {
  onCommentMsg: (msg: CommentMsg) => void;
  onPollMsg: (msg: PollMsg) => void;
}

export const open = ({ onCommentMsg, onPollMsg }: Params) => {
  const ws = new WebSocket(`${WS_URL}`);
  ws.onmessage = event => {
    const data = JSON.parse(event.data);
    const decoded = IncomingMsgDecoder.run(data);
    if (decoded.ok) {
      const msg = decoded.result;
      if (msg.type === "comment") {
        onCommentMsg(msg);
      } else {
        onPollMsg(msg);
      }
    } else {
      console.error(decoded);
    }
  };
};

export type IncomingMsg = CommentMsg | PollMsg;

export interface CommentMsg {
  type: "comment";
  account: string;
  color: string;
  comment: string;
}

export interface PollMsg {
  type: "poll";
  id: string;
  endAt: Moment;
  status: "open" | "closed";
  choices: Record<string, string>;
  stats?: {
    totalVotes: number;
    votePerChoice: Record<string, number>;
  };
}

const CommentMsgDecoder: D.Decoder<CommentMsg> = D.object({
  type: D.constant<"comment">("comment"),
  account: D.string(),
  color: D.string(),
  comment: D.string()
});

const PollMsgDecoder: D.Decoder<PollMsg> = D.object({
  type: D.constant<"poll">("poll"),
  id: D.string(),
  endAt: D.string().map(s => moment(s)),
  status: D.union(D.constant<"open">("open"), D.constant<"closed">("closed")),
  choices: D.dict(D.string()),
  stats: D.optional(
    D.object({
      totalVotes: D.number(),
      votePerChoice: D.dict(D.number())
    })
  )
});

const IncomingMsgDecoder: D.Decoder<IncomingMsg> = D.oneOf<IncomingMsg>(
  CommentMsgDecoder,
  PollMsgDecoder
);
