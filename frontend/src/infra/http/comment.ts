import * as D from "@mojotech/json-type-validation";
import moment, { Moment } from "moment";

import * as http from "./";

export const post = async (arg: {
  contestId: string;
  pollId: string;
  comment: string;
  accessToken: string;
}): Promise<void> =>
  http.post({
    path: `/contests/${arg.contestId}/polls/${arg.pollId}/comments`,
    body: { comment: arg.comment },
    accessToken: arg.accessToken,
    decoder: D.anyJson()
  });
