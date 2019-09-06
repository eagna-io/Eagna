import React, { FC, useState } from "react";
import styled from "styled-components";

import { MarketToken } from "models/market";

import { useMarket } from "../data_provider";
import TokenDetailPage from "./mobile/detail_page";

const TokenListComponent: FC = () => {
  const { market } = useMarket();
  const [selectedToken, setSelectedToken] = useState<MarketToken | null>(null);

  return (
    <>
      <Container>
        {market.attrs.tokens.map(token => {
          return (
            <TokenComponent
              key={token.name}
              token={token}
              onClick={() => setSelectedToken(token)}
            />
          );
        })}
      </Container>
      <TokenDetailPage
        token={selectedToken}
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
  token: MarketToken;
  onClick: () => void;
}

const TokenComponent: FC<TokenComponentProps> = ({ token, onClick }) => {
  const { lmsr, myHistory } = useMarket();
  const tokenPrice = lmsr.computePrice(token.name);
  const amountToken = myHistory ? myHistory.assets.getToken(token.name) : null;
  return (
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
          {amountToken === null ? "-" : amountToken}
        </TokenStatisticsStrong>
        &nbsp;枚
        <br />
        持っています
      </TokenStatistics>
    </TokenContainer>
  );
};

const TokenContainer = styled.div`
  display: inline-block;
  width: calc(50% - 6px);
  margin-top: 20px;
  background: #ffffff;
  box-shadow: 1px 1px 4px 0 rgba(0, 0, 0, 0.5);
  border-radius: 4px;
  vertical-align: top;

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

const TokenSumbnail = styled("div")<{ src: string }>`
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
