import * as D from "@mojotech/json-type-validation";
import moment, { Moment } from "moment";

import { EagnaBackendApi } from "infra/eagna";

export class EagnaPrizeApi {
  static queryAll(): Promise<Prize[]> {
    return EagnaBackendApi.get({
      path: "/prizes/",
      decoder: D.array(PRIZE_DECODER)
    });
  }
}

export interface Prize {
  id: string;
  name: string;
  description: string;
  thumbnailUrl: string;
  point: number;
  available: boolean;
  created: Moment;
}

const PRIZE_DECODER: D.Decoder<Prize> = D.object({
  id: D.string(),
  name: D.string(),
  description: D.string(),
  thumbnailUrl: D.string(),
  point: D.number(),
  available: D.boolean(),
  created: D.string().map(s => moment(s))
});
