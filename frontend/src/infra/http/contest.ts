import * as D from "@mojotech/json-type-validation";
import moment, { Moment } from "moment";

import * as http from "./";

export const getList = async (): Promise<GetListRes> =>
  http.get({ path: "/contests", decoder: GetListResDecoder });

interface GetListRes {
  contests: {
    id: string;
    title: string;
    status: "Upcoming" | "Open" | "Closed" | "Archived";
    category: string;
    event_start_at?: Moment;
  }[];
}

const GetListResDecoder: D.Decoder<GetListRes> = D.object({
  contests: D.array(
    D.object({
      id: D.string(),
      title: D.string(),
      status: D.union(
        D.constant<"Upcoming">("Upcoming"),
        D.constant<"Open">("Open"),
        D.constant<"Closed">("Closed"),
        D.constant<"Archived">("Archived")
      ),
      category: D.string(),
      event_start_at: D.optional(D.string().map(s => moment(s)))
    })
  )
});

export const get = async (id: string): Promise<GetRes> =>
  http.get({ path: `/contests/${id}`, decoder: GetResDecoder });

interface GetRes {
  id: string;
  title: string;
  status: "Upcoming" | "Open" | "Closed" | "Archived";
  category: string;
  event_start_at?: Moment;
  polls: {
    id: string;
    status: "Open" | "Closed";
    title: string;
    idx: number;
    created_at: Moment;
    duration_sec: number;
    choices: {
      name: string;
      color: string;
      idx: number;
    }[];
    resolved_choice?: string;
  }[];
}

const GetResDecoder: D.Decoder<GetRes> = D.object({
  id: D.string(),
  title: D.string(),
  status: D.union(
    D.constant<"Upcoming">("Upcoming"),
    D.constant<"Open">("Open"),
    D.constant<"Closed">("Closed"),
    D.constant<"Archived">("Archived")
  ),
  category: D.string(),
  event_start_at: D.optional(D.string().map(s => moment(s))),
  polls: D.array(
    D.object({
      id: D.string(),
      status: D.union(
        D.constant<"Open">("Open"),
        D.constant<"Closed">("Closed")
      ),
      title: D.string(),
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
      resolved_choice: D.optional(D.string())
    })
  )
});

export const post = async (
  accessToken: string,
  title: string,
  category: string,
  event_start_at?: string
): Promise<string> =>
  http.post({
    path: "/contests",
    body: { title, category, event_start_at },
    accessToken,
    decoder: D.string()
  });
