import React, {FC} from 'react';
import styled from 'styled-components';
import {Link} from 'react-router-dom';

import {Market} from 'models/market';
import StatusBadge from 'components/status_badge';

interface FeaturedMarketComponentProps {
  market: Market;
}

export const Pc: FC<FeaturedMarketComponentProps> = ({market}) => {
  return (
    <Container to={`/market/${market.id}`}>
      <StatusBadge width={87} height={27} status={market.status} />
      <Title>{market.title}</Title>
      <HR />
      <TokenContainer>
        {market.tokens.map(token => (
          <TokenItem key={token.id}>
            <TokenName>{token.name}</TokenName>
            <TokenPrice>-</TokenPrice>
          </TokenItem>
        ))}
      </TokenContainer>
    </Container>
  );
};

const Container = styled(Link)`
  display: inline-block;
  width: 480px;
  vertical-align: top;
  border: 1px solid #c2c6c9;
  border-radius: 4px;
  margin-top: 50px;
  padding: 30px;
  box-shadow: 0 1px 4px 0 rgba(0, 0, 0, 0.5);

  transition: transform 0.2s linear;

  &:nth-of-type(even) {
    margin-left: 50px;
  }

  &:hover {
    transform: scale(1.01);
  }
`;

const Title = styled.div`
  width: 100%;
  font-size: 20px;
  font-weight: 400;
  margin-top: 10px;
`;

const HR = styled.hr`
  width: 100%;
  border: 0.5px solid #c2c6c9;
  margin: 0;
  margin-top: 18px;
`;

const TokenContainer = styled.div`
  width: 100%;
  padding-top: 12px;
  padding-left: 20px;
  padding-right: 20px;
`;

const TokenItem = styled.div`
  margin-top: 13px;
`;

const TokenName = styled.div`
  display: inline-block;
  width: 80%;
  font-size: 18px;
  font-weight: 400;
`;

const TokenPrice = styled.div`
  display: inline-block;
  vertical-align: top;
  width: 20%;
  margin-top: 13px;
  font-size: 18px;
  font-weight: 400;
  text-align: right;
`;
