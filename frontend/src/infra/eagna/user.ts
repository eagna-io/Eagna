import * as D from "@mojotech/json-type-validation";
import moment, { Moment } from "moment";
import { EagnaBackendApi } from "infra/eagna";

export class EagnaUserApi {
  static queryMe(accessToken: string): Promise<User> {
    return EagnaBackendApi.get({
      path: "/users/me/",
      accessToken: accessToken,
      decoder: userDecoder
    });
  }

  static create(
    user: {
      name: string;
      email: string;
    },
    accessToken: string
  ): Promise<User> {
    return EagnaBackendApi.post({
      path: "/users/",
      accessToken: accessToken,
      decoder: userDecoder,
      body: user
    });
  }
}

export interface User {
  id: string;
  name: string;
  email: string;
  isAdmin: boolean;
  pointHistory: (MarketRewardItem | PrizeTradeItem)[];
}

export interface MarketRewardItem {
  type: "MarketReward";
  point: number;
  time: Moment;
  marketId: string;
}

export interface PrizeTradeItem {
  type: "PrizeTrade";
  point: number;
  time: Moment;
  prizeId: string;
  tradeStatus: "Requested" | "Processed";
}

const marketRewardItemDecoder: D.Decoder<MarketRewardItem> = D.object({
  type: D.constant<"MarketReward">("MarketReward"),
  point: D.number(),
  time: D.string().map(s => moment(s)),
  marketId: D.string()
});

const prizeTraedeItemDecoder: D.Decoder<PrizeTradeItem> = D.object({
  type: D.constant<"PrizeTrade">("PrizeTrade"),
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
  pointHistory: D.array(
    D.union(marketRewardItemDecoder, prizeTraedeItemDecoder)
  )
});
