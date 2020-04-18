import * as D from "@mojotech/json-type-validation";
import moment, { Moment } from "moment";

import * as ws from "infra/ws";
import { Comment, Poll } from "model/poll";

export interface Params {
  contestId: string;
  accessToken: string;
  onComment: (comment: Comment) => void;
  onPoll: (poll: Poll) => void;
  onClosed: (result: ClosedMsg) => void;
}

export const open = ({
  contestId,
  accessToken,
  onComment,
  onPoll,
  onClosed
}: Params): WebSocket => {
  return ws.open({
    path: `/contests/${contestId}/${accessToken}`,
    msgDecoder: IncomingMsgDecoder,
    onMsg: msg => {
      switch (msg.type) {
        case "Comment":
          onComment(msg);
          break;
        case "Poll":
          onPoll(msg);
          break;
        case "Closed":
          onClosed(msg);
          break;
      }
    }
  });
};

type IncomingMsg = CommentMsg | PollMsg | ClosedMsg;

interface CommentMsg {
  type: "Comment";
  account_name: string;
  choice?: string;
  comment: string;
}

interface PollMsg {
  type: "Poll";
  id: string;
  title: string;
  status: "Open" | "Closed";
  idx: number;
  created_at: Moment,
  duration_sec: number;
  choices: {
    name: string;
    color: string;
    idx: number;
  }[];
  resolved_choice?: string;
  stats?: {
    totalVotes: number;
    votePerChoice: Record<string, number>;
  };
}

interface ClosedMsg {
  type: "Closed";
  num_polls: number;
  account_score?: number;
}

const CommentMsgDecoder: D.Decoder<CommentMsg> = D.object({
  type: D.constant<"Comment">("Comment"),
  account_name: D.string(),
  choice: D.optional(D.string()),
  comment: D.string()
});

const PollMsgDecoder: D.Decoder<PollMsg> = D.object({
  type: D.constant<"Poll">("Poll"),
  id: D.string(),
  title: D.string(),
  status: D.union(D.constant<"Open">("Open"), D.constant<"Closed">("Closed")),
  idx: D.number(),
  created_at: D.string().map(s => moment(s)),
  duration_sec: D.number(),
  choices: D.array(
    D.object({
      name: D.string(),
      color: D.string(),
      idx: D.number()
    })
  ),
  resolved_choice: D.optional(D.string()),
  stats: D.optional(
    D.object({
      totalVotes: D.number(),
      votePerChoice: D.dict(D.number())
    })
  )
});

const ClosedMsgDecoder: D.Decoder<ClosedMsg> = D.object({
  type: D.constant<"Closed">("Closed"),
  num_polls: D.number(),
  account_score: D.optional(D.number())
});

const IncomingMsgDecoder: D.Decoder<IncomingMsg> = D.union(
  CommentMsgDecoder,
  PollMsgDecoder,
  ClosedMsgDecoder
);
