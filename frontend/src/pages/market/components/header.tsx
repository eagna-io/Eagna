import React, {FC} from 'react';
import styled from 'styled-components';

import {Market, MarketStatus} from 'models/market';
import * as StatusBadge from 'components/status_badge';

interface MarketHeaderComponentProps {
  market: Market | null;
  className?: string;
}

export const Pc: FC<MarketHeaderComponentProps> = ({market, className}) => {
  const Container = styled.div`
    width: 100vw;
    background-color: #f6f8fa;
    border-bottom: 1px solid #979797;
  `;

  const Contents = styled.div`
    position: relative;
    width: 980px;
    margin: 0 auto;
    padding: 30px 0px;
  `;

  const LeftContents = styled.div`
    width: 800px;
  `;

  const Title = styled.div`
    font-size: 28px;
    color: #0466d6;
    font-weight: 600;
    overflow-wrap: break-word;
  `;

  const Desc = styled.div`
    font-size: 16px;
    color: #24292e;
    font-weight: 300;
    overflow-wrap: break-word;
    margin-top: 18px;
  `;

  const StatusBadgeContainer = styled.div`
    position: absolute;
    width: 87px;
    height: 27px;
    top: 30px;
    right: 0px;
  `;

  const TimeContents = styled.div`
    position: absolute;
    right: 0px;
    bottom: 30px;
  `;

  const TimeItem = styled.div`
    font-size: 11px;
    color: #586069;
    margin-top: 5px;
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

  return (
    <Container className={className}>
      <Contents>
        <LeftContents>
          <Title>{market ? market.title : '-'}</Title>
          <Desc>{market ? market.shortDesc : '-'}</Desc>
        </LeftContents>
        <StatusBadgeContainer>
          <StatusBadge.Pc
            status={market !== null ? market.status : MarketStatus.Upcoming}
          />
        </StatusBadgeContainer>
        <TimeContents>
          <TimeItem>
            <TimeKey>Open :</TimeKey>
            <TimeVal>{market ? market.openTime.fromNow() : '-'}</TimeVal>
          </TimeItem>
          <TimeItem>
            <TimeKey>Close :</TimeKey>
            <TimeVal>{market ? market.closeTime.fromNow() : '-'}</TimeVal>
          </TimeItem>
        </TimeContents>
      </Contents>
    </Container>
  );
};

export const Mobile: FC<MarketHeaderComponentProps> = ({market, className}) => {
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
    width: 50px;
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

  return (
    <Container className={className}>
      <Contents>
        <StatusBadgeContainer>
          <StatusBadge.Mobile
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
