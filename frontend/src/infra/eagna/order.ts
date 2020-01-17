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
}

const orderDecoder: D.Decoder<Order> = D.object({
  tokenName: D.optional(D.string()),
  amountToken: D.number(),
  amountCoin: D.number(),
  time: D.string().map(s => moment(s)),
});
