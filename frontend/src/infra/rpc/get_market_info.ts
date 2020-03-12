import * as D from "@mojotech/json-type-validation";

import { call } from "../rpc";

export interface Params {
  marketId: string;
}

export interface Response {
  id: string;
  title: string;
}

export const getMarkeInfo = (params: Params): Promise<Response> =>
  call("getmarketinfo", params, ResponseDecoder());

const ResponseDecoder = (): D.Decoder<Response> =>
  D.object({
    id: D.string(),
    title: D.string()
  });
