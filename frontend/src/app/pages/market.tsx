import React, {FC, useState, useEffect} from 'react';
import styled from 'styled-components';

import {User} from 'models/user';
import {Market} from 'models/market';
import {Eagna} from 'models/organizer';
import {PriceHistory, MyAssets} from 'models/order';
import {getMarket, getOrders, getMyOrders} from 'api/market';
import {MinPcWidth} from 'app/components/responsive';
import Header from 'app/components/header';
import {withUser, LoginStatus} from 'app/components/user';
import NotFoundPage from 'app/pages/not_found';

import MarketHeader from './market/components/header';
import ParticipateComponent from './market/components/participate';
import TokenListComponent from './market/components/tokens';
import OrganizerComponent from './market/components/organizer';
import CoinsComponent from './market/components/coins';
import PrizeComponent from './market/components/prize';
import DescComponent from './market/components/description';

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

const MarketPageInner: FC<MarketPageInnerProps> = React.memo(
  ({user, market}) => {
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
          <OrganizerComponent organizer={Eagna} />
          <HR />
          <CoinsComponent myAssets={myAssets} />
          <HR />
          <PrizeComponent prizes={market.attrs.prizes} />
          <HR />
          <DescComponent desc={market.attrs.description} />
        </Contents>
      </>
    );
  },
);

const Contents = styled.div`
  width: 90%;
  max-width: ${MinPcWidth}px;
  margin: 0 auto;
`;

const HR = styled.hr`
  border: 0.5px solid #979797;
  margin: 0;
`;