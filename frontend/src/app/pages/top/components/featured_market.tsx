import React, { useState, useEffect } from "react";
import styled from "styled-components";

import { Market, MarketStatus, MarketRepository } from "models/market";
import { pc } from "app/components/responsive";
import MarketCard from "app/components/market_card";

export default () => {
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
          <MarketCard key={m.id} market={m} />
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
  padding: 50px 12px;

  ${pc(`
    width: fit-content;
    max-width: 100%;
    margin: 0 auto;
    padding: 50px 0 2px 20px
    overflow-x: scroll;
    white-space: nowrap;
  `)}
`;
