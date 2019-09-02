import moment, { Moment } from "moment";
import * as D from "@mojotech/json-type-validation";

import { request, Method, Failure, FailureCode } from "api/core";
import {
  Market,
  UpcomingMarket,
  OpenMarket,
  ClosedMarket,
  ResolvedMarket,
  MarketAttributes,
  MarketStatus,
  Token,
  Prize,
  PrizeId
} from "models/market";
import { Order, NormalOrder, CoinSupplyOrder, RewardOrder } from "models/order";

/*
 * ========================
 * Get Market
 * ========================
 */

export class EagnaMarketApi {
  static queryById(id: string): Promise<Market> {
    return EagnaBackendApi.get({
      path: `/markets/${id}/`,
      decoder: marketDecoder
    });
  }

  static queryList(): Promise<Market[]> {
    return EagnaBackendApi.get({
      path: "/markets/",
      decoder: D.array(marketDecoder)
    });
  }

  static queryListOfStatus(statusList: MarketStatus[]): Promise<Market[]> {
    return EagnaBackendApi.get({
      path: "/markets/",
      decoder: D.array(marketDecoder),
      params: {
        status: statusList.map(s => s.toLowerCase())
      }
    });
  }

  static queryListOfMine(accessToken: string): Promise<Market[]> {
    return EagnaBackendApi.get({
      path: "/markets",
      params: {
        participated: true
      },
      accessToken: accessToken,
      decoder: D.array(marketDecoder)
    });
  }

  static create(market: MarketAttrs, accessToken: string): Promise<string> {
    return EagnaBackendApi.post({
      path: "/markets/",
      accessToken: accessToken,
      body: market,
      decoder: D.string()
    });
  }

  static resolve(
    marketId: string,
    resolvedTokenName: string,
    accessToken: string
  ): Promise<string> {
    return EagnaBackendApi.put({
      path: `/markets/${marketId}/`,
      accessToken: accessToken,
      body: {
        status: "Resolved",
        resolvedTokenName: resolvedTokenName
      },
      decoder: D.string()
    });
  }

  static async getOrders(marketId: string): Promise<NormalOrder[]> {
    const res = await request({
      method: Method.GET,
      path: `/markets/${marketId}/orders/`,
      decoder: D.object({
        orders: D.array(orderDecoder)
      })
    });
    return res.orders;
  }

  static async getMyOrders(
    marketId: string,
    accessToken: string
  ): Promise<Order[]> {
    const res = await EagnaBackendApi.get({
      path: `/markets/${marketId}/orders/`,
      params: {
        mine: true
      },
      accessToken: accessToken,
      decoder: D.object({
        orders: D.array(orderDecoder)
      })
    });
    return res.orders;
  }

  static createInitialSupplyOrder(
    marketId: string,
    accessToken: string
  ): Promise<Order> {
    return EagnaBackendApi.post({
      path: `/markets/${marketId}/orders/`,
      accessToken: accessToken,
      body: {
        amountToken: 0, // Dont care
        amountCoin: 10000, // Dont care
        time: moment(), // Dont care
        type: "CoinSupply"
      },
      decoder: orderDecoder
    });
  }

  static createNormalOrder(
    marketId: string,
    accessToken: string,
    order: NormalOrder
  ): Promise<NormalOrder> {
    return EagnaBackendApi.post({
      path: `/markets/${marketId}/orders/`,
      accessToken: accessToken,
      body: {
        tokenName: order.tokenName,
        amountToken: order.amountToken,
        amountCoin: order.amountCoin,
        time: order.time,
        type: "Normal"
      },
      decoder: orderDecoder
    });
  }
}

export interface Market {
  id: string;
  attrs: MarketAttrs;
  status: MarketStatus;
  tokenDistribution: Record<string, number>;
}

export interface MarketAttrs {
  title: string;
  organizerId: string;
  description: string;
  open: Moment;
  close: Moment;
  lmsrB: number;
  resolvedTokenName?: string;
  tokens: MarketToken[];
  prizes: MarketPrize[];
}

export enum MarketStatus {
  Upcoming = "Upcoming",
  Open = "Open",
  Closed = "Closed",
  Resolved = "Resolved"
}

export interface MarketToken {
  name: string;
  description: string;
  sumbnailUrl: string;
}

export interface MarketPrize {
  id: number;
  name: string;
  target: string;
  sumbnailUrl: string;
}

const marketDecoder: D.Decoder<Market> = D.object({
  id: D.string(),
  title: D.string(),
  organizerId: D.string(),
  description: D.string(),
  open: D.string().map(s => moment(s)),
  close: D.string().map(s => moment(s)),
  lmsrB: D.number(),
  status: D.string().map(str2status),
  resolvedTokenName: D.optional(D.string()),
  tokenDistribution: D.dict(D.number()).map(dic => Object.entries(dic)),
  tokens: D.array(
    D.object({
      name: D.string(),
      description: D.string(),
      sumbnailUrl: D.string()
    })
  ),
  prizes: D.array(
    D.object({
      id: D.number(),
      name: D.string(),
      target: D.string(),
      sumbnailUrl: D.string()
    })
  )
});

function str2status(s: string): MarketStatus {
  switch (s) {
    case "Upcoming":
      return MarketStatus.Upcoming;
    case "Open":
      return MarketStatus.Open;
    case "Closed":
      return MarketStatus.Closed;
    case "Resolved":
      return MarketStatus.Resolved;
    default:
      throw new Error(`Invalid market status : ${s}`);
  }
}

interface Order {
  token_name?: string;
  amount_token: number;
  amount_coin: number;
  time: Moment;
  type: OrderType;
}

export enum OrderType {
  Normal = "Normal",
  CoinSupply = "CoinSupply",
  Reward = "Reward"
}

const orderDecoder: D.Decoder<Order> = D.object({
  tokenName: D.optional(D.string()),
  amountCoin: D.number(),
  time: D.string().map(s => moment(s)),
  type: D.string().map(s => str2orderType)
});

function str2orderType(s: string): OrderType {
  switch (s) {
    case "Normal":
      return OrderType.Normal;
    case "CoinSupply":
      return OrderType.CoinSupply;
    case "Reward":
      return OrderType.Reward;
    default:
      throw new Error(`Invalid order type : ${s}`);
  }
}
