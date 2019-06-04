import React, {FC, useState, useEffect} from 'react';
import styled from 'styled-components';
import {History} from 'history';

import Header from 'components/header';
import MarketHeader from './market/header';
import TokensComponent from './market/tokens';
// import OrderComponent from './market/order';
// import AssetsComponent from './market/assets';
// import ResultComponent from './market/result';
import HistoryComponent from './market/history';
import DescComponent from './market/description';
import {
  Market,
  MarketId,
  MarketStatus,
  Order,
  NormalOrder,
  getTokenDistribution,
  getTokenPrices,
} from 'models/market';
import User from 'models/user';
import {getMarket, getMarketOrders} from 'api/market';

interface MarketPageProps {
  history: History;
  user: User | null;
  marketId: MarketId;
}

type MarketLike = MarketId | Market;

const MarketPage: FC<MarketPageProps> = ({history, user, marketId}) => {
  const [market, setMarket] = useState<Market | null>(null);
  const [orders, setOrders] = useState<NormalOrder[] | null>(null);
  const [myOrders, setMyOrders] = useState<Order[] | null>(null);

  useEffect(() => {
    getMarket(marketId).then(m => {
      setMarket(m);
    });
  }, [marketId]);

  useEffect(() => {
    getMarketOrders(marketId, user ? user.accessToken : undefined).then(res => {
      setOrders(res.orders);
      setMyOrders(res.myOrders || null);
    });
  }, [user]);

  let tokenDistribution = null;
  let tokenPrices = null;
  if (market && orders) {
    tokenDistribution = getTokenDistribution(market, orders);
    tokenPrices = getTokenPrices(market.lmsrB, tokenDistribution);
  }

  return (
    <>
      <Page>
        <Header history={history} user={user} />
        <MarketHeader market={market} />
        <Contents>
          <StyledTokensComponent
            tokens={market ? market.tokens : []}
            tokenPrices={tokenPrices}
          />
          {market && myOrders && market.status === MarketStatus.Open ? (
            <>
              <StyledHistoryComponent
                tokens={market.tokens}
                myOrders={myOrders || []}
              />
            </>
          ) : null}
          {market &&
          (market.status === MarketStatus.Closed ||
            market.status === MarketStatus.Settled) ? (
            <>
              <StyledHistoryComponent
                tokens={market.tokens}
                myOrders={myOrders || []}
              />
            </>
          ) : null}
          <Description content={market ? market.description : ''} />
        </Contents>
      </Page>
    </>
  );
};

export default MarketPage;

const Page = styled.div`
  width: 100vw;
  background-color: white;
`;

const Contents = styled.div`
  width: 90%;
  max-width: 980px;
  margin: 0 auto;
  padding-bottom: 50px;
`;

const StyledTokensComponent = styled(TokensComponent)`
  margin-top: 50px;
`;

const OrderContainer = styled.div`
  display: flex;
  justify-content: space-between;
  margin-top: 50px;
`;

const StyledHistoryComponent = styled(HistoryComponent)`
  width: 100%;
  margin-top: 50px;
`;

const Description = styled(DescComponent)`
  margin-top: 50px;
`;
