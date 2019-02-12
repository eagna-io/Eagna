import React from 'react';
import styled from 'styled-components';

export default function MarketHeader(props) {
  return (
    <Container className={props.className}>
      <Contents>
        <TitleAndDesc>
          <Title>{props.title}</Title>
          <Desc>{props.desc}</Desc>
        </TitleAndDesc>
        <TimeContents>
          <TimeItem>
            <TimeKey>Open</TimeKey>
            <TimeVal>{props.openTime}</TimeVal>
          </TimeItem>
          <TimeItem>
            <TimeKey>Close</TimeKey>
            <TimeVal>{props.closeTime}</TimeVal>
          </TimeItem>
        </TimeContents>
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
  width: 980px;
  margin: 0 auto;
`;

const TitleAndDesc = styled.div`
  display: inline-block;
  width: 800px;
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

const TimeContents = styled.div`
  display: inline-block;
  width: 180px;
`;

const TimeItem = styled.div`
  width: 100%;
  font-family: Lucida Grande;
  font-size: 11px;
  color: #586069;
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
