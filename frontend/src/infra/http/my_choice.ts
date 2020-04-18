import * as D from "@mojotech/json-type-validation";

import * as http from "./";

export const put = async (arg: {
  contestId: string;
  pollId: string;
  choice: string;
  accessToken: string;
}): Promise<void> =>
  http.put({
    path: `/contests/${arg.contestId}/polls/${arg.pollId}/my_choice`,
    body: { choice: arg.choice },
    accessToken: arg.accessToken,
    decoder: D.anyJson()
  });
