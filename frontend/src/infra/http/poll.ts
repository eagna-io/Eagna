import * as D from "@mojotech/json-type-validation";
import moment, { Moment } from "moment";

import * as http from "./";

export const get = async (): Promise<GetRes> =>
  http.get({ path: "/contest/poll/current", decoder: GetResDecoder });

interface GetRes {
  id: string;
  idx: number;
  title: string;
  endAt: Moment;
  status: "open" | "closed";
  choices: Record<string, string>;
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
  endAt: D.string().map(s => moment(s)),
  status: D.union(D.constant<"open">("open"), D.constant<"closed">("closed")),
  choices: D.dict(D.string()),
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
