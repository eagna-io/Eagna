import React, { FC, useState, useEffect } from "react";
import styled from "styled-components";
import { Link } from "react-router-dom";

import { Market, MarketStatus, MarketRepository } from "models/market";
import { pc, MinPcWidth } from "app/components/responsive";
import StatusBadge from "app/components/status_badge";

const FeaturedMarketListComponent: FC = () => {
  const [markets, setMarkets] = useState<Market[]>([]);

  useEffect(() => {
    MarketRepository.queryListOfStatus([
      MarketStatus.Upcoming,
      MarketStatus.Open
    ]).then(markets => setMarkets(markets.map(({ market }) => market)));
  }, []);

  return (
    <Container>
      <SectionTitle>注目のマーケット</SectionTitle>
      <MarketList>
        {markets.map(m => (
          <MarketComponent key={m.id} market={m} />
        ))}
      </MarketList>
    </Container>
  );
};

const Container = styled.div`
  width: 100vw;
  padding-top: 30px;
  padding-bottom: 50px;
  background-color: #f8f8f8;

  ${pc(`
    padding-top: 64px;
    padding-bottom: 183px;
  `)}
`;

const SectionTitle = styled.h3`
  margin: 0;
  padding: 0;
  width: 100%;
  height: 30px;
  text-align: center;
  line-height: 30px;
  font-size: 20px;
  font-weight: normal;

  ${pc(`
    height: 54px;
    line-height: 54px;
    font-size: 36px;
  `)}
`;

const MarketList = styled.div`
  width: 100%;

  ${pc(`
    width: ${MinPcWidth}px;
    margin: 0 auto;
  `)}
`;

const MarketComponent: FC<{ market: Market }> = React.memo(({ market }) => {
  return (
    <MarketContainer to={`/market/${market.id}`}>
      <StatusBadge status={market.status} />
      <Title>{market.attrs.title}</Title>
      <HR />
      <TokenContainer>
        {market.attrs.tokens.map(token => (
          <TokenItem key={token.name}>
            <TokenName>{token.name}</TokenName>
            <TokenPrice>-</TokenPrice>
          </TokenItem>
        ))}
      </TokenContainer>
    </MarketContainer>
  );
});

export default FeaturedMarketListComponent;

const MarketContainer = styled(Link)`
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
    width: calc((100% - 50px) / 2);
    padding: 30px;
    vertical-align: top;
    transition: transform 0.2s linear;

    &:nth-of-type(even) {
      margin-left: 50px;
    }

    &:first-of-type {
      margin-top: 45px;
    }

    &:hover {
      transform: scale(1.01);
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
