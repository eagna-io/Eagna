import React, {FC} from 'react';
import styled from 'styled-components';

import {Market, MarketStatus} from 'models/market';
import StatusBadge from 'components/status_badge';

interface MarketHeaderComponentProps {
  market: Market | null;
  className?: string;
}

const Header: FC<MarketHeaderComponentProps> = ({market, className}) => {
  return (
    <Container className={className}>
      <Contents>
        <StatusBadgeContainer>
          <StatusBadge
            status={market !== null ? market.status : MarketStatus.Upcoming}
          />
        </StatusBadgeContainer>
        <Title>{market ? market.title : '-'}</Title>
        <Desc>{market ? market.shortDesc : '-'}</Desc>
        <TimeItem>
          <TimeKey>Open :</TimeKey>
          <TimeVal>{market ? market.openTime.fromNow() : '-'}</TimeVal>
        </TimeItem>
        <TimeItem>
          <TimeKey>Close :</TimeKey>
          <TimeVal>{market ? market.closeTime.fromNow() : '-'}</TimeVal>
        </TimeItem>
      </Contents>
    </Container>
  );
};

export default Header;

const Container = styled.div`
  width: 100vw;
  background-color: #f6f8fa;
  border-bottom: 1px solid #979797;
`;

const Contents = styled.div`
  position: relative;
  position: relative;
  width: 90%;
  margin: 0 auto;
  padding: 20px 0px;
`;

const StatusBadgeContainer = styled.div`
  width: 70px;
  height: 27px;
`;

const Title = styled.div`
  margin-top: 10px;
  font-size: 20px;
  color: #0466d6;
  font-weight: 600;
  overflow-wrap: break-word;
`;

const Desc = styled.div`
  font-size: 12px;
  color: #24292e;
  font-weight: 300;
  overflow-wrap: break-word;
  margin-top: 7px;
`;

const TimeItem = styled.div`
  display: inline-block;
  width: 50%;
  font-size: 11px;
  color: #586069;
  margin-top: 12px;
`;

const TimeKey = styled.div`
  display: inline-block;
  font-weight: bold;
`;

const TimeVal = styled.div`
  display: inline-block;
  font-weight: normal;
  margin-left: 10px;
`;
