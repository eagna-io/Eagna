import React, {FC} from 'react';
import styled from 'styled-components';

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
import {MarketStatus} from 'models/market';

import {MarketPageInternalProps} from '../market';

const MarketPage: FC<MarketPageInternalProps> = ({history, user, market}) => {
  if (market === null) {
    return (
      <>
        <Header.Pc history={history} user={user} />
        <h2>ローディング中...</h2>
      </>
    );
  } else {
    return (
      <>
        <Header.Pc history={history} user={user} />
        <MarketHeader.Pc market={market.data} />
        <Contents>
          <StyledChartComponent
            tokens={market.data.tokens}
            lmsrB={market.data.lmsrB}
            startTime={market.data.openTime}
            orders={market.orders}
          />
          <TokensComponent.Pc
            tokens={market.data.tokens}
            tokenPrices={market.tokenPrices}
          />
          <OrderContainer>
            {market.data.status === MarketStatus.Open ? (
              user === null ? (
                <h4>HOGE</h4>
              ) : market.myOrders.length === 0 ? (
                <JoinButtonComponent.Pc
                  requestJoin={() => market.requestInitialSupply(user)}
                />
              ) : (
                <OrderComponent.Pc
                  tokens={market.data.tokens}
                  lmsrB={market.data.lmsrB}
                  tokenDistribution={market.tokenDistribution}
                  myAssets={market.myAssets}
                  requestOrder={(token, amountToken, amountCoin) =>
                    market.requestOrder({
                      user: user,
                      token: token,
                      amountToken: amountToken,
                      amountCoin: amountCoin,
                    })
                  }
                />
              )
            ) : market.data.status === MarketStatus.Closed ||
              market.data.status === MarketStatus.Resolved ? (
              <ResultComponent.Pc
                settleToken={
                  market.data.settleTokenId === undefined
                    ? undefined
                    : market.data.tokens.find(
                        t => t.id === market.data.settleTokenId,
                      )
                }
              />
            ) : null}
            <AssetsComponent.Pc
              tokens={market.data.tokens}
              myAssets={market.myAssets}
            />
          </OrderContainer>
          <HistoryComponent.Pc
            tokens={market.data.tokens}
            myOrders={market.myOrders}
          />
          <DescComponent.Pc content={market.data.description} />
        </Contents>
      </>
    );
  }
};

export default MarketPage;

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
