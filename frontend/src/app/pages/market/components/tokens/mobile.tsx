import React, {FC, useState} from 'react';
import styled from 'styled-components';

import {Token} from 'models/market';

import {TokenListComponentProps} from '../tokens';
import TokenDetailPage from './mobile/detail_page';

const TokenListComponent: FC<TokenListComponentProps> = ({
  market,
  priceHistory,
  myAssets,
}) => {
  const [selectedToken, setSelectedToken] = useState<Token | null>(null);

  return (
    <>
      <Container>
        {market.attrs.tokens.map(token => {
          const tokenPrice = market.tokenPrices.getUncheck(token.name);
          if (tokenPrice === undefined) {
            throw new Error(`${token.name} does not exist in tokenPrice`);
          }
          const amountToken = myAssets
            ? myAssets.getTokenUncheck(token.name)
            : null;
          return (
            <TokenComponent
              key={token.name}
              token={token}
              tokenPrice={tokenPrice}
              amountToken={amountToken}
              onClick={() => setSelectedToken(token)}
            />
          );
        })}
      </Container>
      <TokenDetailPage
        token={selectedToken}
        market={market}
        priceHistory={priceHistory}
        myAssets={myAssets}
        onClose={() => setSelectedToken(null)}
      />
    </>
  );
};

export default TokenListComponent;

const Container = styled.div`
  padding: 20px 0 40px 0;
`;

interface TokenComponentProps {
  token: Token;
  tokenPrice: number;
  amountToken: number | null;
  onClick: () => void;
}

const TokenComponent: FC<TokenComponentProps> = ({
  token,
  tokenPrice,
  amountToken,
  onClick,
}) => {
  return (
    <>
      <TokenContainer onClick={onClick}>
        <TokenName>{token.name}</TokenName>
        <TokenSumbnail src={token.sumbnailUrl} />
        <TokenStatistics>
          <TokenStatisticsStrong>{tokenPrice}</TokenStatisticsStrong>
          <br />
          coin&nbsp;/&nbsp;枚
        </TokenStatistics>
        <TokenStatistics>
          <TokenStatisticsStrong>
            {amountToken === null ? '-' : amountToken}
          </TokenStatisticsStrong>
          &nbsp;枚
          <br />
          持っています
        </TokenStatistics>
      </TokenContainer>
    </>
  );
};

const TokenContainer = styled.div`
  display: inline-block;
  width: calc(50% - 6px);
  margin-top: 20px;
  background: #ffffff;
  box-shadow: 1px 1px 4px 0 rgba(0, 0, 0, 0.5);
  border-radius: 4px;

  &:nth-of-type(odd) {
    margin-right: 12px;
  }
`;

const TokenName = styled.h4`
  margin: 0;
  padding: 9px 6px;
  font-size: 13px;
  font-weight: normal;
`;

const TokenSumbnail = styled('div')<{src: string}>`
  width: 100%;
  height: 80px;
  background-image: url(${props => props.src});
  background-size: cover;
  background-position: center;
`;

const TokenStatistics = styled.div`
  display: inline-block;
  vertical-align: top;
  width: 50%;
  padding: 15px 15px 15px 5px;
  text-align: right;
  font-size: 8px;
  font-weight: normal;
`;

const TokenStatisticsStrong = styled.strong`
  font-size: 16px;
  font-weight: bold;
`;
