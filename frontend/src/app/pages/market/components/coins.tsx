import React from "react";
import styled from "styled-components";

import { pc } from "app/components/responsive";

import { useMarket } from "./data_provider";
import Section from "./section_skelton";

const CoinsComponent: React.FC = () => {
  const { myHistory } = useMarket();

  return (
    <Section title="所持コイン">
      <AssetContainer>
        <CoinIcon src="/img/market/coins.svg" />
        <Volume>
          {myHistory ? myHistory.assets.getCoin() : "-"}
          <VolumeUnit>&nbsp;coins</VolumeUnit>
        </Volume>
      </AssetContainer>
    </Section>
  );
};

export default CoinsComponent;

const AssetContainer = styled.div`
  width: 100%;
  text-align: right;
`;

const CoinIcon = styled.img`
  width: 28px;
  height: 28px;
  margin-right: 20px;
  vertical-align: top;

  ${pc(`
    width: 56px;
    height: 56px;
    margin-right: 40px;
  `)}
`;

const Volume = styled.p`
  display: inline-block;
  padding: 0;
  margin: 0;
  font-weight: bold;
  font-size: 20px;
  text-align: right;

  ${pc(`
    font-size: 30px;
  `)}
`;

const VolumeUnit = styled.span`
  font-weight: normal;
  font-size: 14px;

  ${pc(`
    font-size: 20px;
  `)}
`;
