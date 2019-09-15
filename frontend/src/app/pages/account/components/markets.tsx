import React, { FC } from "react";
import styled from "styled-components";

import { Market } from "models/market";
import { pc } from "app/components/responsive";
import MarketCard from "app/components/market_card";

interface MarketsComponentProps {
  title: string;
  markets: Market[];
}

const MarketsComponent: FC<MarketsComponentProps> = ({ title, markets }) => {
  return (
    <Container>
      <Title>{title}</Title>
      <MarketsContainer>
        {markets.map(market => (
          <MarketCard key={market.id} market={market} />
        ))}
      </MarketsContainer>
    </Container>
  );
};

export default MarketsComponent;

const Container = styled.div`
  width: 100%;
  margin-top: 50px;

  ${pc(`
    margin-top: 90px;
  `)}
`;

const Title = styled.h3`
  margin: 0;
  padding: 0;
  font-size: 15px;
  font-weight: bold;
`;

const MarketsContainer = styled.div`
  width: 100%;

  ${pc(`
    white-space: nowrap;
    overflow: scroll;
    padding: 2px;
  `)}
`;
