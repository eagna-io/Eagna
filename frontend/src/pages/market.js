import React, { useState, useEffect, useContext } from 'react';
import styled from 'styled-components';

import { AccessTokenContext } from 'src/context';
import { getMarket, postOrder, InvalidAccessTokenError, MarketNotFoundError, TokenPriceIsMovedError } from 'src/api';

import Loading from 'src/components/loading';
import NoticeBar from 'src/components/notice_bar';
import Header from 'src/components/header';
import MarketHeader from './market/header';
import TokensComponent from './market/tokens';
import OrderComponent from './market/order';
import AssetsComponent from './market/assets';
import ResultComponent from './market/result';
import HistoryComponent from './market/history';
import DescComponent from './market/description';


export default function MarketPage(props) {
  const marketId = props.match.params.id;
  const {accessToken, setAccessToken} = useContext(AccessTokenContext);
  const [market, setMarket] = useState(null);
  const [loading, setLoading] = useState(false);
  const [[errMsg, errNonce], setErr] = useState([null, null]);

  useEffect(() => {
    setLoading(true);
    getMarket(marketId, accessToken)
      .then(market => {
        setMarket(market);
        setLoading(false);
      })
      .catch(err => {
        switch(err) {
          case InvalidAccessTokenError:
            setAccessToken(null);
            setErr(["Your login session is expired", Date.now()]);
            setMarket(null);
            setLoading(false);
            break;
          case MarketNotFoundError:
            setErr(["Market not found", Date.now()]);
            setMarket(null);
            setLoading(false);
            break;
          default:
            console.error(err);
            break;
        }
      });
  }, [marketId, accessToken]);

  const requestOrder = (token, orderType, amountToken, amountCoin) => {
    if (orderType === "buy") {
      // 十分なCoinを持っているかチェック
      if (market.me.orderHistory.currentAmountCoin() + amountCoin < 0) {
        setErr(["You don't have enough coin", Date.now()]);
        return;
      }
    } else {
      // 十分な対象Tokenを持っているかチェック
      if (market.me.orderHistory.currentAmountToken(token) + amountToken < 0) {
        setErr(["You don't have enough token", Date.now()]);
        return;
      }
    }

    setLoading(true);
    postOrder(token.id, amountToken, amountCoin, accessToken)
      .then(() => setErr(["Success!!", Date.now()]))
      .catch(err => {
        switch(err) {
          case InvalidAccessTokenError:
            setAccessToken(null);
            setErr(["You login session is expired", Date.now()]);
            setMarket(null);
            break;
          case TokenPriceIsMovedError:
            setErr(["Token price is changed. Please try again", Date.now()]);
            break;
        }
      })
      // リクエストの結果にかかわらず market 情報を再取得する
      .then(() => getMarket(market.id, accessToken))
      .then(market => setMarket(market))
      .finally(() => setLoading(false));
  };

  return (
    <>
    <Loading loading={loading} />
    <NoticeBar nonce={errNonce}>{errMsg}</NoticeBar>
    <Page>
      <Header />
      <MarketHeader market={market} />
      <Contents>
        <Tokens tokens={market ? market.tokens : []} />
        {
          market && market.me && market.status === "open" ? (
            <OrderContainer>
              <OrderComponent
                tokens={market.tokens}
                requestOrder={requestOrder}
              />
              <AssetsComponent
                tokens={market.tokens}
                orderHistory={market.me.orderHistory}
              />
            </OrderContainer>
          ) : null
        }
        {
          market && (
            market.status === "closed" || market.status === "settled"
          ) ? (
            <OrderContainer>
              <ResultComponent settleToken={market.settleToken} />
              {
                market.me ? (
                  <AssetsComponent
                    tokens={market.tokens}
                    orderHistory={market.me.orderHistory}
                  />
                ) : null
              }
            </OrderContainer>
          ) : null
        }
        {
          market && market.me ? (
            <History
              tokens={market.tokens}
              orderHistory={market.me.orderHistory}
            />
          ) : null
        }
        <Description content={market ? market.desc : ""}/>
      </Contents>
    </Page>
    </>
  )
}


const Page = styled.div`
  width: 100vw;
  background-color: white;
`;

const Contents = styled.div`
  width: 980px;
  margin: 0 auto;
  padding-bottom: 50px;
`;

const Tokens = styled(TokensComponent)`
  margin-top: 50px;
`;

const OrderContainer = styled.div`
  display: flex;
  justify-content: space-between;
  margin-top: 50px;
`;

const History = styled(HistoryComponent)`
  width: 100%;
  margin-top: 50px;
`;

const Description = styled(DescComponent)`
  margin-top: 50px;
`;
