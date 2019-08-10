import React, {FC, useState, useEffect, useMemo} from 'react';
import styled from 'styled-components';

import {User} from 'models/user';
import {Market} from 'models/market';
import {PriceHistory, MyAssets, NormalOrder, Order} from 'models/order';
import {getMarket, getOrders, getMyOrders} from 'api/market';
import {MinPcWidth} from 'app/components/responsive';
import Header from 'app/components/header';
import {withUser, LoginStatus} from 'app/components/user';
import NotFoundPage from 'app/pages/not_found';

import MarketHeader from './market/components/header';
import ParticipateComponent from './market/components/participate';
import TokenListComponent from './market/components/tokens';

interface MarketPageProps {
  user: LoginStatus;
  marketId: string;
}

const MarketPage: FC<MarketPageProps> = ({user, marketId}) => {
  const [market, setMarket] = useState<'Loading' | Market | null>('Loading');

  useEffect(() => {
    getMarket(marketId).then(market => {
      setMarket(market);
    });
  }, [marketId]);

  if (market === 'Loading') {
    return (
      <>
        <Header />
        <h2>Loading...</h2>
      </>
    );
  } else if (market === null) {
    return <NotFoundPage />;
  } else {
    return <MarketPageInner market={market} user={user} />;
  }
};

export default withUser(MarketPage);

interface MarketPageInnerProps {
  user: LoginStatus;
  market: Market;
}

const MarketPageInner: FC<MarketPageInnerProps> = ({user, market}) => {
  const [priceHistory, setPriceHistory] = useState<PriceHistory | null>(null);
  const [myAssets, setMyAssets] = useState<MyAssets | null>(null);

  useEffect(() => {
    getOrders(market.id).then(orders => {
      setPriceHistory(new PriceHistory(market, orders));
    });
  }, [market]);

  useEffect(() => {
    if (user instanceof User) {
      user
        .getAccessToken()
        .then(accessToken => getMyOrders(market.id, accessToken as string))
        .then(orders => {
          if (orders === 'Unauthorized') {
            console.log('Unauthorized');
          } else {
            if (orders.length === 0) {
              setMyAssets(null);
            } else {
              setMyAssets(new MyAssets(market.attrs.tokens, orders));
            }
          }
        });
    }
  }, [user, market]);

  return (
    <>
      <Header />
      <MarketHeader market={market} />
      <Contents>
        {myAssets === null ? (
          <>
            <ParticipateComponent market={market} />
            <HR />
          </>
        ) : null}
        <TokenListComponent
          market={market}
          priceHistory={priceHistory}
          myAssets={myAssets}
        />
        <HR />
      </Contents>
    </>
  );
};

const Contents = styled.div`
  width: 90%;
  max-width: ${MinPcWidth}px;
  margin: 0 auto;
`;

const HR = styled.hr`
  border: 0.5px solid #979797;
  margin: 0;
`;
