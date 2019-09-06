import moment, { Moment } from "moment";
import * as D from "@mojotech/json-type-validation";

import { EagnaBackendApi } from "../eagna";

export class EagnaOrderApi {
  static async queryList(marketId: string): Promise<Order[]> {
    const res = await EagnaBackendApi.get({
      path: `/markets/${marketId}/orders/`,
      decoder: D.object({
        orders: D.array(orderDecoder)
      })
    });
    return res.orders;
  }

  static async queryListOfMine(
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

  static create(
    marketId: string,
    accessToken: string,
    order: Order
  ): Promise<Order> {
    return EagnaBackendApi.post({
      path: `/markets/${marketId}/orders/`,
      accessToken: accessToken,
      body: order,
      decoder: orderDecoder
    });
  }
}

export interface Order {
  tokenName?: string;
  amountToken: number;
  amountCoin: number;
  time: Moment;
  type: OrderType;
}

export enum OrderType {
  CoinSupply = "CoinSupply",
  Normal = "Normal",
  Reward = "Reward"
}

const orderDecoder: D.Decoder<Order> = D.object({
  tokenName: D.optional(D.string()),
  amountToken: D.number(),
  amountCoin: D.number(),
  time: D.string().map(s => moment(s)),
  type: D.string().map(str2orderType)
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
