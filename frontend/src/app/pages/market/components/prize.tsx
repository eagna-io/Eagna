import React from 'react';
import styled from 'styled-components';

import {Prize} from 'models/market';
import Section from './section_skelton';

interface Props {
  prizes: Prize[];
}

const PrizeComponent: React.FC<Props> = ({prizes}) => {
  return (
    <Section title="報酬">
      {prizes.map(prize => (
        <PrizeContainer>
          <PrizeSumbnail src={prize.sumbnailUrl} />
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
`;

const PrizeInfo = styled.div`
  display: inline-block;
  margin-left: 25px;
  vertical-align: top;
`;

const PrizeName = styled.h3`
  margin: 0;
  padding: 0;
  font-weight: bold;
  font-size: 15px;
`;

const PrizeTarget = styled.p`
  margin: 0;
  padding: 0;
  margin-top: 12px;
  font-size: 10px;
`;
