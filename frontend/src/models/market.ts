import { Moment } from "moment";

import {
  EagnaMarketApi,
  Market as InfraMarket,
  MarketAttrs as InfraMarketAttrs,
  MarketStatus as InfraMarketStatus,
  MarketToken as InfraMarketToken,
  MarketPrize as InfraMarketPrize
} from "infra/eagna/market";
import { User } from "models/user";
import { LMSR } from "models/lmsr";

/*
 * ====================
 *      Market
 * ====================
 */
export class Market {
  constructor(
    readonly id: string,
    readonly attrs: MarketAttrs,
    readonly status: MarketStatus
  ) {}

  static fromInfra(market: InfraMarket): Market {
    return new Market(
      market.id,
      MarketAttrs.fromInfra(market.attrs),
      fromInfraMarketStatus(market.status)
    );
  }
}

export class MarketAttrs {
  constructor(
    readonly title: string,
    readonly organizerId: string,
    readonly description: string,
    readonly open: Moment,
    readonly close: Moment,
    readonly lmsrB: number,
    readonly tokens: MarketToken[],
    readonly prizes: MarketPrize[],
    readonly resolvedTokenName?: string
  ) {}

  static fromInfra(attrs: InfraMarketAttrs): MarketAttrs {
    return new MarketAttrs(
      attrs.title,
      attrs.organizerId,
      attrs.description,
      attrs.open,
      attrs.close,
      attrs.lmsrB,
      attrs.tokens.map(MarketToken.fromInfra),
      attrs.prizes.map(MarketPrize.fromInfra),
      attrs.resolvedTokenName
    );
  }
}

export enum MarketStatus {
  Upcoming = "Upcoming",
  Open = "Open",
  Closed = "Closed",
  Resolved = "Resolved"
}

function fromInfraMarketStatus(status: InfraMarketStatus): MarketStatus {
  switch (status) {
    case InfraMarketStatus.Upcoming:
      return MarketStatus.Upcoming;
    case InfraMarketStatus.Open:
      return MarketStatus.Open;
    case InfraMarketStatus.Closed:
      return MarketStatus.Closed;
    case InfraMarketStatus.Resolved:
      return MarketStatus.Resolved;
  }
}

export class MarketToken {
  constructor(
    readonly name: string,
    readonly description: string,
    readonly thumbnailUrl: string
  ) {}

  static fromInfra(token: InfraMarketToken): MarketToken {
    return new MarketToken(token.name, token.description, token.thumbnailUrl);
  }
}

export class MarketPrize {
  constructor(
    readonly id: number,
    readonly name: string,
    readonly target: string,
    readonly thumbnailUrl: string
  ) {}

  static fromInfra(prize: InfraMarketPrize): MarketPrize {
    return new MarketPrize(
      prize.id,
      prize.name,
      prize.target,
      prize.thumbnailUrl
    );
  }
}

export class TokenDistribution {
  constructor(readonly rawDistribution: { name: string; amount: number }[]) {}

  get(tokenName: string): number {
    const token = this.rawDistribution.find(({ name }) => name === tokenName);
    if (!token) {
      throw new Error(`Token ${tokenName} is not found`);
    }
    return token.amount;
  }

  lmsr(lmsrB: number): LMSR {
    return new LMSR(this.rawDistribution, lmsrB);
  }

  add(tokenName: string, tokenAmount: number): TokenDistribution {
    const distribution = this.clone();
    distribution.addAssign(tokenName, tokenAmount);
    return distribution;
  }

  addAssign(tokenName: string, tokenAmount: number) {
    this.rawDistribution.forEach(({ name, amount }, idx, array) => {
      if (name === tokenName) {
        array[idx].amount = amount + tokenAmount;
      }
    });
  }

  clone(): TokenDistribution {
    return new TokenDistribution(
      this.rawDistribution.map(({ name, amount }) => ({ name, amount }))
    );
  }
}

/*
 * ===============
 * Repository
 * ===============
 */
export class MarketRepository {
  static async queryById(
    id: string
  ): Promise<{ market: Market; distribution: TokenDistribution }> {
    const infraMarket = await EagnaMarketApi.queryById(id);
    console.log(infraMarket);
    return MarketRepository.convertInfraMarket(infraMarket);
  }

  static async queryList(): Promise<
    { market: Market; distribution: TokenDistribution }[]
  > {
    const infraMarketList = await EagnaMarketApi.queryList();
    return infraMarketList.map(MarketRepository.convertInfraMarket);
  }

  static async queryListOfStatus(
    statusList: MarketStatus[]
  ): Promise<{ market: Market; distribution: TokenDistribution }[]> {
    const infraMarketList = await EagnaMarketApi.queryListOfStatus(statusList);
    return infraMarketList.map(MarketRepository.convertInfraMarket);
  }

  static async queryListOfMine(
    user: User
  ): Promise<{ market: Market; distribution: TokenDistribution }[]> {
    const accessToken = await user.getAccessToken();
    const infraMarketList = await EagnaMarketApi.queryListOfMine(accessToken);
    return infraMarketList.map(MarketRepository.convertInfraMarket);
  }

  static convertInfraMarket(
    infraMarket: InfraMarket
  ): { market: Market; distribution: TokenDistribution } {
    const market = Market.fromInfra(infraMarket);
    const rawDistribution = Object.entries(infraMarket.tokenDistribution).map(
      ([name, amount]) => ({
        name,
        amount
      })
    );
    const distribution = new TokenDistribution(rawDistribution);
    return {
      market,
      distribution
    };
  }

  static async create(market: MarketAttrs, user: User): Promise<string> {
    const accessToken = await user.getAccessToken();
    return await EagnaMarketApi.create(market, accessToken);
  }

  static async resolve(
    market: Market,
    resolvedTokenName: string,
    user: User
  ): Promise<string> {
    const accessToken = await user.getAccessToken();
    return await EagnaMarketApi.resolve(
      market.id,
      resolvedTokenName,
      accessToken
    );
  }
}
