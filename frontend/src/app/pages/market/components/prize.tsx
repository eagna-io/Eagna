import React from "react";
import styled from "styled-components";

import { pc } from "app/components/responsive";

import { useMarket } from "./data_provider";
import Section from "./section_skelton";

const PrizeComponent: React.FC = () => {
  const { market } = useMarket();

  return (
    <Section title="報酬">
      {market.attrs.prizes.map(prize => (
        <PrizeContainer key="prize.id">
          <PrizeSumbnail src={prize.thumbnailUrl} />
          <PrizeInfo>
            <PrizeName>{prize.name}</PrizeName>
            <PrizeTarget>{prize.target}</PrizeTarget>
          </PrizeInfo>
        </PrizeContainer>
      ))}
    </Section>
  );
};

export default PrizeComponent;

const PrizeContainer = styled.div`
  width: 100%;
  margin-top: 20px;

  &:first-of-type {
    margin-top: 0px;
  }
`;

const PrizeSumbnail = styled.img`
  display: inline-block;
  width: 95px;
  height: 95px;

  ${pc(`
    width: 210px;
    height: 210px;
  `)}
`;

const PrizeInfo = styled.div`
  display: inline-block;
  width: calc(100% - 95px - 25px);
  margin-left: 25px;
  vertical-align: top;

  ${pc(`
    width: calc(100% - 210px - 50px);
    margin-left: 50px;
  `)}
`;

const PrizeName = styled.h3`
  margin: 0;
  padding: 0;
  font-weight: bold;
  font-size: 15px;

  ${pc(`
    font-size: 21px;
  `)}
`;

const PrizeTarget = styled.p`
  margin: 0;
  padding: 0;
  margin-top: 12px;
  font-size: 10px;

  ${pc(`
    font-size: 18px;
  `)}
`;
