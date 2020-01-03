import * as D from "@mojotech/json-type-validation";
import moment, { Moment } from "moment";
import { EagnaBackendApi } from "infra/eagna";

export class EagnaUserApi {
  static queryMe(accessToken: string): Promise<User | null> {
    return EagnaBackendApi.get({
      path: "/users/me/",
      accessToken: accessToken,
      decoder: userDecoder
    }).catch(e => null);
  }

  static createAccessToken(args: {
    email: string;
    password: string;
  }): Promise<string | null> {
    return EagnaBackendApi.post({
      path: "/users/me/access_token/",
      decoder: D.object({ token: D.string() }),
      body: args
    })
      .then(({ token }) => token)
      .catch(e => null);
  }

  // 新規ユーザー登録を行う
  // 既に登録済みの場合はnull
  static create(user: {
    name: string;
    password: string;
    invitationToken: string;
  }): Promise<string | null> {
    return EagnaBackendApi.post({
      path: "/users/me/",
      decoder: D.object({ token: D.string() }),
      body: user
    })
      .then(({ token }) => token)
      .catch(e => null);
  }
}

export interface User {
  id: string;
  name: string;
  email: string;
  isAdmin: boolean;
  coin: number;
  point: number;
}

export interface MarketRewardRecord {
  point: number;
  time: Moment;
  marketId: string;
}

export interface PrizeTradeRecord {
  point: number;
  time: Moment;
  prizeId: string;
  tradeStatus: "Requested" | "Processed";
}

const marketRewardItemDecoder: D.Decoder<MarketRewardRecord> = D.object({
  point: D.number(),
  time: D.string().map(s => moment(s)),
  marketId: D.string()
});

const prizeTraedeItemDecoder: D.Decoder<PrizeTradeRecord> = D.object({
  point: D.number(),
  time: D.string().map(s => moment(s)),
  prizeId: D.string(),
  tradeStatus: D.union(
    D.constant<"Requested">("Requested"),
    D.constant<"Processed">("Processed")
  )
});

const userDecoder: D.Decoder<User> = D.object({
  id: D.string(),
  name: D.string(),
  email: D.string(),
  isAdmin: D.boolean(),
  coin: D.number(),
  point: D.number(),
});
