import React, { FC } from "react";
import styled from "styled-components";

import { MarketToken, MarketStatus } from "models/market";

import { useMarket } from "../data_provider";
import ChartComponent from "./chart";
import AssetComponent from "./asset";
import OrderComponent from "./order";
import ResolvedMark from "./resolved_mark";

const TokenListComponent: FC = () => {
  const { market } = useMarket();
  return (
    <Container>
      {market.attrs.tokens.map(token => {
        return <TokenComponent key={token.name} token={token} />;
      })}
    </Container>
  );
};

export default TokenListComponent;

const Container = styled.div`
  padding: 20px 0 40px 0;
`;

interface TokenComponentProps {
  token: MarketToken;
}

const TokenComponent: FC<TokenComponentProps> = ({ token }) => {
  const { market } = useMarket();
  const isResolved =
    market.status === MarketStatus.Resolved &&
    market.attrs.resolvedTokenName === token.name;

  return (
    <>
      <TokenContainer>
        <TokenSumbnail src={token.sumbnailUrl} />
        <TokenContents>
          {isResolved ? <ResolvedMark /> : null}
          <TokenName>{token.name}</TokenName>
          <TokenDesc>{token.description}</TokenDesc>
          <ChartComponent token={token} />
          <AssetComponent token={token} />
          <OrderComponent token={token} />
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

const TokenSumbnail = styled("div")<{ src: string }>`
  width: 100%;
  height: 250px;
  background-image: url(${props => props.src});
  background-size: cover;
  background-position: center;
`;

const TokenContents = styled.div`
  position: relative;
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
