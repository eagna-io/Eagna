import * as D from "@mojotech/json-type-validation";
import moment, { Moment } from "moment";

import * as http from "./";

export const get = async (): Promise<GetRes> =>
  http.get({ path: "/contest/poll/current", decoder: GetResDecoder });

interface GetRes {
  id: string;
  idx: number;
  title: string;
  status: "Open" | "Closed";
  created_at: Moment;
  duration_sec: number;
  choices: {
    name: string;
    color: string;
    idx: number;
  }[];
  resolved?: string;
  stats?: {
    totalVotes: number;
    votePerChoice: Record<string, number>;
  };
}

const GetResDecoder: D.Decoder<GetRes> = D.object({
  id: D.string(),
  idx: D.number(),
  title: D.string(),
  status: D.union(D.constant<"Open">("Open"), D.constant<"Closed">("Closed")),
  created_at: D.string().map(s => moment(s)),
  duration_sec: D.number(),
  choices: D.array(
    D.object({
      name: D.string(),
      color: D.string(),
      idx: D.number()
    })
  ),
  resolved: D.optional(D.string()),
  stats: D.optional(
    D.object({
      totalVotes: D.number(),
      votePerChoice: D.dict(D.number())
    })
  )
});

export const post = async (args: {
  contestId: string;
  title: string;
  durationSec: number;
  choices: { name: string; color: string; idx: number }[];
  accessToken: string;
}): Promise<string> =>
  http.post({
    path: `/contests/${args.contestId}/polls`,
    body: {
      title: args.title,
      duration_sec: args.durationSec,
      choices: args.choices
    },
    accessToken: args.accessToken,
    decoder: D.string()
  });

export const resolve = async (arg: {
  contestId: string;
  pollId: string;
  choice: string;
  accessToken: string;
}): Promise<void> =>
  http.patch({
    path: `/contests/${arg.contestId}/polls/${arg.pollId}`,
    body: { resolved_choice: arg.choice },
    accessToken: arg.accessToken,
    decoder: D.anyJson()
  });
