import React, {FC, useState, useEffect, useCallback} from 'react';
import {History} from 'history';

import {User, getAccessToken} from 'models/user';
import {
  Market,
  MarketId,
  Token,
  PublicOrderHistory,
  MyOrderHistory,
  TokenDistribution,
  TokenPrices,
  MyAssets,
  newTokenDistribution,
  newTokenPrices,
  getMyAssets,
} from 'models/market';
import {
  getMarket,
  getOrders,
  createInitialSupplyOrder,
  createNormalOrder,
} from 'api/market';
import {Pc, Tablet, Mobile} from 'components/responsive';
import PcMarketPage from './market/pc';
import MobileMarketPage from './market/mobile';

export interface RequestOrderArgs {
  user: User;
  token: Token;
  amountToken: number;
  amountCoin: number;
}

export interface MarketPageInternalProps {
  history: History;
  user: User | null;
  market: {
    data: Market;
    orders: PublicOrderHistory;
    myOrders: MyOrderHistory;
    tokenDistribution: TokenDistribution;
    tokenPrices: TokenPrices;
    myAssets: MyAssets;
    requestOrder: (args: RequestOrderArgs) => void;
    requestInitialSupply: (user: User) => void;
  } | null;
}

interface MarketPageProps {
  history: History;
  user: User | null;
  marketId: MarketId;
}

const MarketPage: FC<MarketPageProps> = ({history, user, marketId}) => {
  const [market, setMarket] = useState<Market | null>(null);
  const [orders, setOrders] = useState<{
    orders: PublicOrderHistory;
    myOrders: MyOrderHistory;
  } | null>(null);

  useEffect(() => {
    getMarket(marketId).then(m => {
      setMarket(m);
    });
  }, [marketId]);

  useEffect(() => {
    if (user) {
      getAccessToken(user).then(accessToken => {
        if (accessToken === null) {
          history.push('/login');
        } else {
          getOrders(marketId, accessToken).then(res => {
            setOrders({orders: res.orders, myOrders: res.myOrders || []});
          });
        }
      });
    } else {
      getOrders(marketId).then(res => {
        setOrders({orders: res.orders, myOrders: res.myOrders || []});
      });
    }
  }, [marketId, user, history]);

  const requestOrder: (args: RequestOrderArgs) => void = useCallback(
    ({user, token, amountToken, amountCoin}) => {
      getAccessToken(user).then(accessToken => {
        if (accessToken === null) {
          alert(
            'ログインセッションが切れました。再ログインをお願いいたします。',
          );
          history.push('/login');
          // TODO
          // 再ログイン後に自動的にオーダーを出す
        } else {
          createNormalOrder({
            marketId: marketId,
            order: {
              tokenId: token.id,
              amountToken: amountToken,
              amountCoin: amountCoin,
            },
            accessToken: accessToken,
          })
            .then(res => {
              if (res === 'PriceSlip') {
                alert(
                  '指定された価格でオーダーが通りませんでした。\n' +
                    '改めてオーダーをお願いいたします。',
                );
              } else {
                alert(
                  'オーダーに成功しました！\n' +
                    `トークン   : ${token.name}\n` +
                    `トークン数 : ${amountToken}\n` +
                    `コイン数   : ${res.amountCoin}`,
                );
              }
              return getOrders(marketId, accessToken);
            })
            .then(res => {
              setOrders({orders: res.orders, myOrders: res.myOrders || []});
            });
        }
      });
    },
    [marketId, setOrders, history],
  );

  const requestInitialSupply: (user: User) => void = useCallback(
    user => {
      getAccessToken(user).then(accessToken => {
        if (accessToken === null) {
          alert(
            'ログインセッションが切れています。再ログインをお願いいたします。',
          );
          history.push('/login');
        } else {
          createInitialSupplyOrder({
            marketId: marketId,
            accessToken: accessToken,
          })
            .then(() => getOrders(marketId, accessToken))
            .then(res => {
              setOrders({orders: res.orders, myOrders: res.myOrders || []});
            });
        }
      });
    },
    [marketId, history],
  );

  let marketDatas = null;
  if (market !== null && orders !== null) {
    const tokenDistribution = newTokenDistribution(
      market.tokens,
      orders.orders,
    );
    const tokenPrices = newTokenPrices(market.lmsrB, tokenDistribution);
    const myAssets = getMyAssets(market.tokens, orders.myOrders);
    marketDatas = {
      data: market,
      tokenDistribution: tokenDistribution,
      tokenPrices: tokenPrices,
      myAssets: myAssets,
      requestOrder: requestOrder,
      requestInitialSupply: requestInitialSupply,
      ...orders,
    };
  }

  return (
    <>
      <Pc>
        <PcMarketPage history={history} user={user} market={marketDatas} />
      </Pc>
      <Tablet>
        <MobileMarketPage history={history} user={user} market={marketDatas} />
      </Tablet>
      <Mobile>
        <MobileMarketPage history={history} user={user} market={marketDatas} />
      </Mobile>
    </>
  );
};

export default MarketPage;
