import React from 'react';
import styled from 'styled-components';

import { dateToStr } from 'models/time';


export default function MarketHeader(props) {
  const market = props.market;

  return (
    <Container className={props.className}>
      <Contents>
        <LeftContents>
          <Title>{market ? market.title : "-"}</Title>
          <Desc>{market ? market.shortDesc : "-"}</Desc>
        </LeftContents>
        <RightContents>
          <Status>{market ? market.status : "-" }</Status>
          <TimeContents>
            <TimeItem>
              <TimeKey>Open</TimeKey>
              <TimeVal>{market ? dateToStr(market.openTime) : "-"}</TimeVal>
            </TimeItem>
            <TimeItem>
              <TimeKey>Close</TimeKey>
              <TimeVal>{market ? dateToStr(market.closeTime) : "-"}</TimeVal>
            </TimeItem>
          </TimeContents>
        </RightContents>
      </Contents>
    </Container>
  );
}

const Container = styled.div`
  width: 100vw;
  height: 130px;
  background-color: #F6F8FA;
  border-bottom: 1px solid #979797;
`;

const Contents = styled.div`
  width: 100%;
  display: flex;
  flex-wrap: wrap;
  justify-content: space-around;
  max-width: 980px;
  margin: 0 auto;
`;

const LeftContents = styled.div`
  width: 100%;
  max-width: 800px;
`;

const RightContents = styled.div`
  width: 100%;
  max-width: 180px;
`;

const Title = styled.div`
  font-size: 28px;
  color: #0466D6;
  font-family: Hiragino Sans;
  font-weight: 600;
  margin-top: 27px;
`;

const Desc = styled.div`
  font-size: 16px;
  color: #24292E;
  font-family: Hiragino Sans;
  font-weight: 300;
  margin-top: 18px;
`;

const Status = styled.div`
  font-size: 17px;
  color: #24292E;
  font-family: Hiragino Sans;
  font-weight: bold;
  margin-top: 30px;
`;

const TimeContents = styled.div`
  margin-top: 20px;
`;

const TimeItem = styled.div`
  width: 100%;
  font-family: Lucida Grande;
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
