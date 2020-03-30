import * as D from "@mojotech/json-type-validation";
import moment, { Moment } from "moment";

import * as ws from "infra/ws";
import { Comment, Poll} from "../models";

export interface Params {
  onComment: (comment: Comment) => void;
  onPoll: (poll: Poll) => void;
}

export const open = ({ onComment, onPoll}: Params) => {
  ws.open({msgDecoder: IncomingMsgDecoder, onMsg: (msg) => {
    if (msg.type === "comment") {
      onComment(msg);
    } else {
      onPoll(msg);
    }
  }});
};

type IncomingMsg = CommentMsg | PollMsg;

interface CommentMsg {
  type: "comment";
  account: string;
  color: string;
  comment: string;
}

interface PollMsg {
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
  status: D.union(D.constant<"open">("open"), D.constant<"closed">("closed")),   choices: D.dict(D.string()),
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
