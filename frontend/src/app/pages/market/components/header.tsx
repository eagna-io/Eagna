import React, {FC} from 'react';
import styled from 'styled-components';
import {Moment} from 'moment';

import {Market} from 'models/market';
import {Eagna} from 'models/organizer';
import {pc, MinPcWidth} from 'app/components/responsive';
import StatusBadge from 'app/components/status_badge';

interface MarketHeaderProps {
  market: Market;
}

const MarketHeader: FC<MarketHeaderProps> = ({market}) => {
  return (
    <Container>
      <StyledStatusBadge status={market.getStatus()} />
      <MarketPeriod open={market.attrs.open} close={market.attrs.close} />
      <MarketTitle>{market.attrs.title}</MarketTitle>
      <MarketCreator>
        マーケット作成者
        <MarketCreatorName>{Eagna.name}</MarketCreatorName>
      </MarketCreator>
    </Container>
  );
};

export default MarketHeader;

const Container = styled.div`
  width: 100%;
  position: relative;
  padding: 25px 20px;
  background: #f6f8fa;
  box-shadow: 0 1px 0 0 #979797;

  ${pc(`
    height: 166px;
  `)}
`;

const StyledStatusBadge = styled(StatusBadge)`
  display: inline-block;
  width: 70px;
  height: 30px;
  font-size: 11px;
  line-height: 30px;

  ${pc(`
    position: absolute;
    top: 30px;
    right: calc((100% - ${MinPcWidth}px) / 2);
    width: 87px;
    height: 27px;
    font-size: 15px;
    font-weight: bold;
  `)}
`;

const MarketPeriod: FC<{open: Moment; close: Moment}> = ({open, close}) => {
  return (
    <MarketPeriodText>
      <MarketPeriodDate>{open.format('M/D')}</MarketPeriodDate>
      {open.format(' (ddd) HH:mm')}
      &nbsp;~&nbsp;
      <MarketPeriodDate>{close.format('M/D')}</MarketPeriodDate>
      {close.format(' (ddd) HH:mm')}
    </MarketPeriodText>
  );
};

const MarketPeriodText = styled.div`
  display: inline-block;
  width: calc(100% - 70px);
  text-align: right;
  font-size: 10px;

  ${pc(`
    position: absolute;
    bottom: 50px;
    right: calc((100% - ${MinPcWidth}px) / 2);
    font-size: 13px;
    font-weight: bold;
  `)}
`;

const MarketPeriodDate = styled.span`
  font-size: 12px;

  ${pc(`
    font-size: 15px;
  `)}
`;

const MarketTitle = styled.h3`
  margin: 0;
  margin-top: 20px;
  padding: 0;
  font-size: 20px;
  font-weight: bold;
  color: #0466d6;

  ${pc(`
    position: absolute;
    top: 30px;
    left: calc((100% - ${MinPcWidth}px) / 2);
    width: ${MinPcWidth - 220}px;
    margin-top: 0px;
    font-size: 28px;
  `)}
`;

const MarketCreator = styled.div`
  width: 100%;
  margin-top: 20px;
  font-size: 10px;
  text-align: right;

  ${pc(`
    position: absolute;
    bottom: 15px;
    right: calc((100% - ${MinPcWidth}px) / 2);
    width: 250px;
    margin-top: 0px;
    font-size: 14px;
  `)}
`;

const MarketCreatorName = styled.strong`
  font-size: 16px;
  font-weight: bold;
  margin-left: 15px;

  ${pc(`
    font-size: 18px;
  `)}
`;
