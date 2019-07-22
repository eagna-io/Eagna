import React, {FC} from 'react';
import styled from 'styled-components';
import {Link} from 'react-router-dom';

import {Market} from 'models/market';
import {pc} from 'components/responsive';
import StatusBadge from 'components/status_badge';

interface Props {
  market: Market;
}

const Component: FC<Props> = React.memo(({market}) => {
  return (
    <Container to={`/market/${market.id}`}>
      <StatusBadge status={market.status} />
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
});

export default Component;

const Container = styled(Link)`
  display: block;
  width: calc(100% - 40px);
  border: 1px solid #c2c6c9;
  border-radius: 4px;
  margin: 0 auto;
  margin-top: 45px;
  padding: 20px;
  box-shadow: 0 1px 4px 0 rgba(0, 0, 0, 0.5);

  &:first-of-type {
    margin-top: 30px;
  }

  ${pc(`
    display: inline-block;
    width: 480px;
    padding: 30px;
    transition: transform 0.2s linear;

    &:nth-of-type(even) {
      margin-left: 50px;
    }

    &:hover {
      transform: scale(0.1);
    }
  `)}
`;

const Title = styled.div`
  width: 100%;
  font-size: 14px;
  font-weight: 400;
  line-height: 21px;
  margin-top: 10px;

  ${pc(`
    font-size: 20px;
    line-height: 30px;
  `)}
`;

const HR = styled.hr`
  width: 100%;
  border: 0.5px solid #c2c6c9;
  margin: 0;
  margin-top: 18px;
`;

const TokenContainer = styled.div`
  width: 100%;
  padding: 0 12px;

  ${pc(`
    padding: 0 20px;
  `)}
`;

const TokenItem = styled.div`
  margin-top: 14px;

  &:first-of-type {
    margin-top: 20px;
  }

  ${pc(`
    &:first-of-type {
      margin-top: 25px;
    }
  `)}
`;

const TokenName = styled.div`
  display: inline-block;
  width: 80%;
  font-size: 12px;
  font-weight: 400;

  ${pc(`
    font-size: 18px;
  `)}
`;

const TokenPrice = styled.div`
  display: inline-block;
  vertical-align: top;
  width: 20%;
  font-size: 12px;
  font-weight: 400;
  text-align: right;

  ${pc(`
    font-size: 18px;
  `)}
`;
