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
