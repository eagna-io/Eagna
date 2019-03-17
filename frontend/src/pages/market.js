import React, { useState, useEffect, useContext } from 'react';
import { Redirect } from 'react-router-dom';
import styled from 'styled-components';

import Header from 'src/components/header';
import MarketHeader from './market/header';
import TokensComponent from './market/tokens';
import OrderComponent from './market/order';
import AssetsComponent from './market/assets';
import ResultComponent from './market/result';
import DescComponent from './market/description';

import { AccessTokenContext } from 'src/context';
import { getMarket, InvalidAccessTokenError, MarketNotFoundError } from 'src/api';


export default function MarketPage(props) {
  const marketId = props.match.params.id;
  const {token, setToken} = useContext(AccessTokenContext);
  const [market, setMarket] = useState(null);
  const [errMsg, setErrMsg] = useState(null);

  useEffect(() => {
    getMarket(marketId, token)
      .then(market => setMarket(market))
      .catch(err => {
        switch(err) {
          case InvalidAccessTokenError:
            setToken(null);
            setMarket(null);
            break;
          case MarketNotFoundError:
            setErrMsg("Market not found");
            setMarket(null);
            break;
        }
      });
  }, [marketId, token]);

  return (
    <Page>
      <Header />
      { errMsg ? <h3>{errMsg}</h3> : null }
      { market ? MarketContents(market) : null }
    </Page>
  )
}

function MarketContents(market) {
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
                marketId={marketId}
                setErrMsg={setErrMsg}
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
