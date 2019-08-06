import React, {FC} from 'react';
import styled from 'styled-components';

import {Market} from 'models/market';

interface MarketsComponentProps {
  title: string;
  markets: Market[];
}

const MarketsComponent: FC<MarketsComponentProps> = ({title, markets}) => {
  return (
    <Container>
      <Title>{title}</Title>
      {markets.map(market => (
        <MarketComponent key={market.id} market={market} />
      ))}
    </Container>
  );
};

export default MarketsComponent;

const Container = styled.div`
  width: 100%;
  margin-top: 50px;
`;

const Title = styled.h3`
  margin: 0;
  padding: 0;
  font-size: 15px;
  font-weight: bold;
`;

const MarketComponent: FC<{market: Market}> = ({market}) => {
  return <MarketContainer />;
};

const MarketContainer = styled.div`
  display: inline-block;
  width: calc(50% - 6px);
  height: 200px;
  box-shadow: 0 0 2px 0 rgba(0, 0, 0, 0.5);
  border-radius: 4px;
  margin-top: 20px;

  &:nth-of-type(odd) {
    margin-right: 12px;
  }
`;
