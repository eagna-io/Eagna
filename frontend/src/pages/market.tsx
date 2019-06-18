import React, {FC, useState, useEffect} from 'react';
import styled from 'styled-components';
import {History} from 'history';

import Header from 'components/header';
import MarketHeader from './market/header';
import ChartComponent from './market/chart';
import TokensComponent from './market/tokens';
import OrderComponent from './market/order';
import AssetsComponent from './market/assets';
import JoinButtonComponent from './market/joinButton';
// import ResultComponent from './market/result';
import HistoryComponent from './market/history';
import DescComponent from './market/description';
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

const MarketPage: FC<MarketPageProps> = ({history, user, marketId}) => {
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
      <Page>
        <Header history={history} user={user} />
        <MarketHeader market={market} />
        <Contents>
          <StyledChartComponent
            tokens={market.tokens}
            lmsrB={market.lmsrB}
            startTime={market.openTime}
            orders={orders}
          />
          <StyledTokensComponent
            tokens={market.tokens}
            tokenPrices={tokenPrices}
          />
          {user && market.status === MarketStatus.Open ? (
            <>
              <OrderContainer>
                {myOrders.length === 0 ? (
                  <StyledJoinButtonComponent
                    requestJoin={() => requestJoin(user)}
                  />
                ) : (
                  <StyledOrderComponent
                    tokens={market.tokens}
                    lmsrB={market.lmsrB}
                    tokenDistribution={tokenDistribution}
                    myAssets={myAssets}
                    requestOrder={(token, amountToken, amountCoin) =>
                      requestOrder(user, token, amountToken, amountCoin)
                    }
                  />
                )}
                <StyledAssetsComponent
                  tokens={market.tokens}
                  myAssets={myAssets}
                  maxHeight={300}
                />
              </OrderContainer>
              <StyledHistoryComponent
                tokens={market.tokens}
                myOrders={myOrders}
                maxHeight={300}
              />
            </>
          ) : null}
          {market &&
          (market.status === MarketStatus.Closed ||
            market.status === MarketStatus.Settled) ? (
            <>
              <OrderContainer>
                {myAssets ? (
                  <StyledAssetsComponent
                    tokens={market.tokens}
                    myAssets={myAssets}
                    maxHeight={300}
                  />
                ) : null}
              </OrderContainer>
              <StyledHistoryComponent
                tokens={market.tokens}
                myOrders={myOrders}
                maxHeight={300}
              />
            </>
          ) : null}
          <Description content={market ? market.description : ''} />
        </Contents>
      </Page>
    </>
  );
};

const LoadingMarketPage: FC<{}> = () => {
  return <div>Loading...</div>;
};

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

const StyledChartComponent = styled(ChartComponent)`
  width: 100%;
  height: 200px;
  margin-top: 50px;
`;

const StyledTokensComponent = styled(TokensComponent)`
  margin-top: 50px;
`;

const OrderContainer = styled.div`
  display: flex;
  justify-content: space-between;
  align-items: start;
`;

const StyledJoinButtonComponent = styled(JoinButtonComponent)`
  margin-top: 50px;
`;

const StyledOrderComponent = styled(OrderComponent)`
  margin-top: 50px;
`;

const StyledAssetsComponent = styled(AssetsComponent)`
  margin-top: 50px;
`;

const StyledHistoryComponent = styled(HistoryComponent)`
  width: 100%;
  margin-top: 50px;
`;

const Description = styled(DescComponent)`
  margin-top: 50px;
`;
