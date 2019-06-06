import React, {FC, useState, useEffect} from 'react';
import styled from 'styled-components';
import {History} from 'history';

import Header from 'components/header';
import MarketHeader from './market/header';
import TokensComponent from './market/tokens';
import OrderComponent from './market/order';
import AssetsComponent from './market/assets';
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
  getTokenDistribution,
  getTokenPrices,
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
      setMyOrders(res.myOrders || null);
    });
  }, [marketId, user]);

  let tokenDistribution = null;
  let tokenPrices = null;
  if (market && orders) {
    tokenDistribution = getTokenDistribution(market.tokens, orders);
    tokenPrices = getTokenPrices(market.lmsrB, tokenDistribution);
  }

  let myAssets = null;
  if (market && myOrders) {
    myAssets = getMyAssets(market.tokens, myOrders);
  }

  const requestOrder = (
    user: User,
    token: Token,
    amountToken: number,
    amountCoin: number,
  ): void => {
    postOrder({
      marketId: marketId,
      order: {
        tokenId: token.id,
        amountToken: amountToken,
        amountCoin: amountCoin,
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
        return getOrders(marketId, user.accessToken);
      })
      .then(res => {
        setOrders(res.orders);
        if (!res.myOrders) {
          throw 'Success to create a new Order, but it is not reflected';
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
          <StyledTokensComponent
            tokens={market ? market.tokens : []}
            tokenPrices={tokenPrices}
          />
          {market &&
          myOrders &&
          myAssets &&
          tokenDistribution &&
          user &&
          market.status === MarketStatus.Open ? (
            <>
              <OrderContainer>
                <StyledOrderComponent
                  tokens={market.tokens}
                  lmsrB={market.lmsrB}
                  tokenDistribution={tokenDistribution}
                  myAssets={myAssets}
                  requestOrder={(token, amountToken, amountCoin) =>
                    requestOrder(user, token, amountToken, amountCoin)
                  }
                />
                <StyledAssetsComponent
                  tokens={market.tokens}
                  myAssets={myAssets}
                />
              </OrderContainer>
              <StyledHistoryComponent
                tokens={market.tokens}
                myOrders={myOrders}
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
                  />
                ) : null}
              </OrderContainer>
              {myOrders ? (
                <StyledHistoryComponent
                  tokens={market.tokens}
                  myOrders={myOrders || []}
                />
              ) : null}
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
