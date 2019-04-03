import React, { useState, useEffect, useContext } from 'react';
import styled from 'styled-components';

import { AccessTokenContext } from 'src/context';
import { getMarket, InvalidAccessTokenError, MarketNotFoundError } from 'src/api';

import Loading from 'src/components/loading';
import NoticeBar from 'src/components/notice_bar';
import Header from 'src/components/header';
import MarketHeader from './market/header';
import TokensComponent from './market/tokens';
import OrderComponent from './market/order';
import AssetsComponent from './market/assets';
import ResultComponent from './market/result';
import DescComponent from './market/description';


export default function MarketPage(props) {
  const marketId = props.match.params.id;
  const {token, setToken} = useContext(AccessTokenContext);
  const [market, setMarket] = useState(null);
  const [loading, setLoading] = useState(false);
  const [[errMsg, errNonce], setErr] = useState([null, null]);

  useEffect(() => {
    setLoading(true);
    getMarket(marketId, token)
      .then(market => {
        setMarket(market);
        setLoading(false);
      })
      .catch(err => {
        switch(err) {
          case InvalidAccessTokenError:
            setToken(null);
            setErr(["Your login session is expired", Date.now()]);
            setMarket(null);
            setLoading(false);
            break;
          case MarketNotFoundError:
            setErr(["Market not found", Date.now()]);
            setMarket(null);
            setLoading(false);
            break;
        }
      });
  }, [marketId, token]);

  return (
    <>
    <Loading loading={loading} />
    <NoticeBar nonce={errNonce}>{errMsg}</NoticeBar>
    <Page>
      <Header />
      { market ? MarketContents(market, setErr, setMarket) : null }
    </Page>
    </>
  )
}

function MarketContents(market, setErr, setMarket) {
  return (
    <>
    <MarketHeader
      title={market.title}
      shortDesc={market.shortDesc}
      openTs={market.openTs}
      closeTs={market.closeTs}
      status={market.status} />
    <Contents>
      <Tokens tokens={market.tokens} />
      { market.me !== undefined ?
          market.status === "open" ? (
            <OrderContainer>
              <OrderComponent
                tokens={market.tokens}
                marketId={market.id}
                setErr={setErr}
                setMarket={setMarket} />
              <AssetsComponent
                tokens={market.tokens}
                assets={market.me.tokens}
                coins={market.me.coins} />
            </OrderContainer>
          ) : market.status === "closed" ? (
            <OrderContainer>
              <ResultComponent result={market.result} />
              <AssetsComponent
                tokens={market.tokens}
                assets={market.me.tokens}
                coins={market.me.coins} />
            </OrderContainer>
          ) : null
        : null
      }
      <Description content={market.desc}/>
    </Contents>
    </>
  );
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

const Description = styled(DescComponent)`
  margin-top: 50px;
`;
