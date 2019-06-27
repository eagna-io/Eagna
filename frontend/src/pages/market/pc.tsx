import React, {FC, useState, useEffect} from 'react';
import styled from 'styled-components';
import {History} from 'history';

import * as Header from 'components/header';
import * as MarketHeader from './components/header';
import ChartComponent from './components/chart';
import * as TokensComponent from './components/tokens';
import * as OrderComponent from './components/order';
import * as AssetsComponent from './components/assets';
import * as JoinButtonComponent from './components/join_button';
import * as ResultComponent from './components/result';
import * as HistoryComponent from './components/history';
import * as DescComponent from './components/description';
import {
  Market,
  MarketId,
  MarketStatus,
  Token,
  PublicOrderHistory,
  MyOrderHistory,
  newTokenDistribution,
  newTokenPrices,
  getMyAssets,
} from 'models/market';
import User from 'models/user';
import {getMarket, getOrders, postOrder} from 'api/market';

interface MarketPageProps {
  history: History;
  user: User | null;
  marketId: MarketId;
}

export const MarketPage: FC<MarketPageProps> = ({history, user, marketId}) => {
  const [market, setMarket] = useState<Market | null>(null);
  const [orders, setOrders] = useState<PublicOrderHistory | null>(null);
  const [myOrders, setMyOrders] = useState<MyOrderHistory | null>(null);

  useEffect(() => {
    getMarket(marketId).then(m => {
      setMarket(m);
    });
  }, [marketId]);

  useEffect(() => {
    getOrders(marketId, user ? user.accessToken : undefined).then(res => {
      setOrders(res.orders);
      setMyOrders(res.myOrders || []);
    });
  }, [marketId, user]);

  return (
    <>
      {market != null && orders != null && myOrders != null ? (
        <LoadedMarketPage
          market={market}
          orders={orders}
          setOrders={setOrders}
          myOrders={myOrders}
          setMyOrders={setMyOrders}
          user={user}
          history={history}
        />
      ) : (
        <LoadingMarketPage />
      )}
    </>
  );
};

export default MarketPage;

interface LoadedMarketPageProps {
  market: Market;
  orders: PublicOrderHistory;
  setOrders(orders: PublicOrderHistory): void;
  myOrders: MyOrderHistory;
  setMyOrders(myOrders: MyOrderHistory): void;
  user: User | null;
  history: History;
}

const LoadedMarketPage: FC<LoadedMarketPageProps> = ({
  market,
  orders,
  setOrders,
  myOrders,
  setMyOrders,
  user,
  history,
}) => {
  const tokenDistribution = newTokenDistribution(market.tokens, orders);
  const tokenPrices = newTokenPrices(market.lmsrB, tokenDistribution);
  const myAssets = getMyAssets(market.tokens, myOrders);

  const requestOrder = (
    user: User,
    token: Token,
    amountToken: number,
    amountCoin: number,
  ): void => {
    postOrder({
      marketId: market.id,
      order: {
        tokenId: token.id,
        amountToken: amountToken,
        amountCoin: amountCoin,
        type: 'normal',
      },
      accessToken: user.accessToken,
    })
      .then(res => {
        const settledAmountCoin = res.amountCoin;
        alert(
          'Orderに成功しました！\n' +
            `トークン   : ${token.name}\n` +
            `トークン数 : ${amountToken}\n` +
            `コイン数   : ${settledAmountCoin}`,
        );
        return getOrders(market.id, user.accessToken);
      })
      .then(res => {
        setOrders(res.orders);
        if (!res.myOrders) {
          throw 'Logic error : Success to create a new Order, but it is not reflected';
        } else {
          setMyOrders(res.myOrders);
        }
      });
  };

  const requestJoin = (user: User) => {
    postOrder({
      marketId: market.id,
      order: {
        type: 'initialSupply',
      },
      accessToken: user.accessToken,
    })
      .then(() => getOrders(market.id, user.accessToken))
      .then(res => {
        setOrders(res.orders);
        if (!res.myOrders) {
          throw 'Logic error : Success to create a new Order, but it is not reflected';
        } else {
          setMyOrders(res.myOrders);
        }
      });
  };

  return (
    <>
      <Header.Pc history={history} user={user} />
      <MarketHeader.Pc market={market} />
      <Contents>
        <StyledChartComponent
          tokens={market.tokens}
          lmsrB={market.lmsrB}
          startTime={market.openTime}
          orders={orders}
        />
        <TokensComponent.Pc tokens={market.tokens} tokenPrices={tokenPrices} />
        {user && market.status === MarketStatus.Open ? (
          <>
            <OrderContainer>
              {myOrders.length === 0 ? (
                <JoinButtonComponent.Pc requestJoin={() => requestJoin(user)} />
              ) : (
                <OrderComponent.Pc
                  tokens={market.tokens}
                  lmsrB={market.lmsrB}
                  tokenDistribution={tokenDistribution}
                  myAssets={myAssets}
                  requestOrder={(token, amountToken, amountCoin) =>
                    requestOrder(user, token, amountToken, amountCoin)
                  }
                />
              )}
              <AssetsComponent.Pc tokens={market.tokens} myAssets={myAssets} />
            </OrderContainer>
            <HistoryComponent.Pc tokens={market.tokens} myOrders={myOrders} />
          </>
        ) : null}
        {market &&
        (market.status === MarketStatus.Closed ||
          market.status === MarketStatus.Resolved) ? (
          <>
            <OrderContainer>
              <ResultComponent.Pc
                settleToken={
                  market.settleTokenId === undefined
                    ? undefined
                    : market.tokens.find(t => t.id === market.settleTokenId)
                }
              />
              {myAssets ? (
                <AssetsComponent.Pc
                  tokens={market.tokens}
                  myAssets={myAssets}
                />
              ) : null}
            </OrderContainer>
            <HistoryComponent.Pc tokens={market.tokens} myOrders={myOrders} />
          </>
        ) : null}
        <DescComponent.Pc content={market ? market.description : ''} />
      </Contents>
    </>
  );
};

export const LoadingMarketPage: FC<{}> = () => {
  return <div />;
};

const Contents = styled.div`
  width: 90%;
  max-width: 980px;
  margin: 0 auto;
  padding-bottom: 50px;
`;

const StyledChartComponent = styled(ChartComponent)`
  width: 100%;
  height: 200px;
  margin-top: 50px;
`;

const OrderContainer = styled.div`
  display: flex;
  justify-content: space-between;
  align-items: start;
`;
