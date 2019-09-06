import React from "react";

import { User } from "models/user";
import { LMSR } from "models/lmsr";
import { Market, TokenDistribution, MarketRepository } from "models/market";
import {
  Order,
  DistributionHistory,
  PriceHistory,
  MyAssets,
  OrderRepository
} from "models/order";

interface MarketContext {
  market: Market;
  distribution: TokenDistribution;
  lmsr: LMSR;
  publicHistory: {
    orders: Order[];
    distribution: DistributionHistory;
    price: PriceHistory;
  };
  myHistory?: {
    orders: Order[];
    assets: MyAssets;
  };
}

const MarketContextObject = React.createContext<MarketContext | null>(null);

export function useMarket(): MarketContext {
  return React.useContext(MarketContextObject) as MarketContext;
}

interface ComponentProps {
  marketId: string;
  user: User | null;
  loadingView: React.ReactElement;
  notfoundView: React.ReactElement;
}

const DataProvider: React.FC<ComponentProps> = ({
  marketId,
  user,
  children,
  loadingView,
  notfoundView
}) => {
  const [dataSource, setMarket] = React.useState<
    "Loading" | "Notfound" | MarketDataSource
  >("Loading");

  React.useEffect(() => {
    MarketDataSource.queryPublic(marketId).then(setMarket);
  }, [marketId]);

  React.useEffect(() => {
    if (
      dataSource instanceof MarketDataSource &&
      !dataSource.myOrders &&
      user instanceof User
    ) {
      dataSource.queryPrivate(user).then(setMarket);
    }
  }, [dataSource, user]);

  const marketContext = React.useMemo(() => {
    if (dataSource instanceof MarketDataSource) {
      return dataSource.computeMarketContext();
    } else {
      return null;
    }
  }, [dataSource]);

  console.log(marketContext);

  if (dataSource === "Loading") {
    return loadingView;
  } else if (dataSource === "Notfound") {
    return notfoundView;
  } else {
    return (
      <MarketContextObject.Provider value={marketContext}>
        {children}
      </MarketContextObject.Provider>
    );
  }
};

export default DataProvider;

class MarketDataSource {
  constructor(
    readonly market: Market,
    readonly distribution: TokenDistribution,
    readonly publicOrders: Order[],
    readonly myOrders?: Order[]
  ) {}

  static async queryPublic(
    marketId: string
  ): Promise<MarketDataSource | "Notfound"> {
    try {
      const { market, distribution } = await MarketRepository.queryById(
        marketId
      );
      const publicOrders = await OrderRepository.queryList(market);
      return new MarketDataSource(market, distribution, publicOrders);
    } catch (e) {
      return "Notfound";
    }
  }

  async queryPrivate(user: User): Promise<MarketDataSource> {
    const myOrders = await OrderRepository.queryListOfMine(this.market, user);
    return new MarketDataSource(
      this.market,
      this.distribution,
      this.publicOrders,
      myOrders
    );
  }

  computeMarketContext(): MarketContext {
    const distributionHistory = DistributionHistory.fromPublicOrders(
      this.market,
      this.publicOrders
    );
    const priceHistory = PriceHistory.fromDistributionHistory(
      distributionHistory,
      this.market.attrs.lmsrB
    );
    return {
      market: this.market,
      distribution: this.distribution,
      lmsr: this.distribution.lmsr(this.market.attrs.lmsrB),
      publicHistory: {
        orders: this.publicOrders,
        distribution: distributionHistory,
        price: priceHistory
      },
      myHistory: this.myOrders
        ? {
            orders: this.myOrders,
            assets: MyAssets.fromMyOrders(this.myOrders)
          }
        : undefined
    };
  }
}
