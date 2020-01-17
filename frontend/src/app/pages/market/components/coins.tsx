import React from "react";
import styled from "styled-components";
import { useSelector } from "react-redux";

import { pc } from "app/components/responsive";
import { RootState } from "app/redux";

import Section from "./section_skelton";

const CoinsComponent: React.FC = () => {
  const user = useSelector((state: RootState) => state.user.user);

  return (
    <Section title="所持チップ">
      <AssetContainer>
        <CoinIcon src="/img/market/coins.svg" />
        <Volume>
          {user ? user.coin : "-"}
          <VolumeUnit>&nbsp;枚</VolumeUnit>
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
