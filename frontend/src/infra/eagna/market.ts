import moment, { Moment } from "moment";
import * as D from "@mojotech/json-type-validation";

import { EagnaBackendApi } from "../eagna";

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
      path: "/markets/",
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
  thumbnailUrl: string;
}

export interface MarketPrize {
  id: number;
  name: string;
  target: string;
  thumbnailUrl: string;
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
  tokenDistribution: D.dict(D.number()),
  tokens: D.array(
    D.object({
      name: D.string(),
      description: D.string(),
      thumbnailUrl: D.string()
    })
  ),
  prizes: D.array(
    D.object({
      id: D.number(),
      name: D.string(),
      target: D.string(),
      thumbnailUrl: D.string()
    })
  )
}).map(
  ({
    id,
    title,
    organizerId,
    description,
    open,
    close,
    lmsrB,
    status,
    resolvedTokenName,
    tokenDistribution,
    tokens,
    prizes
  }) => ({
    id,
    attrs: {
      title,
      organizerId,
      description,
      open,
      close,
      lmsrB,
      tokens,
      prizes,
      resolvedTokenName
    },
    status,
    tokenDistribution
  })
);

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