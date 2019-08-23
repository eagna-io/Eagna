import React, {FC} from 'react';
import styled from 'styled-components';

import {Market, Token} from 'models/market';
import {MyAssets, PriceHistory} from 'models/order';

import {TokenListComponentProps} from '../tokens';
import ChartComponent from './chart';
import AssetComponent from './asset';
import OrderComponent from './order';

const TokenListComponent: FC<TokenListComponentProps> = ({
  market,
  priceHistory,
  myAssets,
}) => {
  return (
    <Container>
      {market.attrs.tokens.map(token => {
        const tokenPrice = market.tokenPrices.getUncheck(token.name);
        if (tokenPrice === undefined) {
          throw new Error(`${token.name} does not exist in tokenPrice`);
        }
        return (
          <TokenComponent
            key={token.name}
            market={market}
            token={token}
            tokenPrice={tokenPrice}
            myAssets={myAssets}
            priceHistory={priceHistory}
          />
        );
      })}
    </Container>
  );
};

export default TokenListComponent;

const Container = styled.div`
  padding: 20px 0 40px 0;
`;

interface TokenComponentProps {
  market: Market;
  token: Token;
  tokenPrice: number;
  myAssets: MyAssets | null;
  priceHistory: PriceHistory | null;
}

const TokenComponent: FC<TokenComponentProps> = ({
  market,
  token,
  tokenPrice,
  myAssets,
  priceHistory,
}) => {
  return (
    <>
      <TokenContainer>
        <TokenSumbnail src={token.sumbnailUrl} />
        <TokenContents>
          <TokenName>{token.name}</TokenName>
          <TokenDesc>{token.desc}</TokenDesc>
          <ChartComponent token={token} priceHistory={priceHistory} />
          <AssetComponent
            amountToken={myAssets ? myAssets.getTokenUncheck(token.name) : null}
            amountCoin={myAssets ? myAssets.getCoin() : null}
          />
          <OrderComponent token={token} market={market} myAssets={myAssets} />
        </TokenContents>
      </TokenContainer>
    </>
  );
};

const TokenContainer = styled.div`
  display: inline-block;
  width: calc(50% - 20px);
  margin-top: 20px;
  background: #ffffff;
  box-shadow: 1px 1px 4px 0 rgba(0, 0, 0, 0.5);
  border-radius: 4px;
  vertical-align: top;

  &:nth-of-type(odd) {
    margin-right: 40px;
  }
`;

const TokenSumbnail = styled('div')<{src: string}>`
  width: 100%;
  height: 250px;
  background-image: url(${props => props.src});
  background-size: cover;
  background-position: center;
`;

const TokenContents = styled.div`
  width: 100%;
  padding: 30px 22px;
`;

const TokenName = styled.h4`
  margin: 0;
  font-size: 22px;
  font-weight: bold;
`;

const TokenDesc = styled.h4`
  margin: 0;
  margin-top: 15px;
  font-size: 14px;
  font-weight: normal;
  color: #979797;
`;
