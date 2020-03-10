import * as D from "@mojotech/json-type-validation";

import { call } from "../rpc";

export interface Params {
  marketId: string;
  outcomeId: string;
  accountName: string;
}

export const vote = (params: Params): Promise<void> =>
  call("vote", params, D.succeed(undefined));
