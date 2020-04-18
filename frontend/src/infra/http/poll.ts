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
  created_at: Moment,
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

export const resolve = async (choice: string): Promise<void> =>
  http.patch({
    path: "/contest/poll",
    body: { resolved: choice },
    decoder: D.anyJson()
  });
