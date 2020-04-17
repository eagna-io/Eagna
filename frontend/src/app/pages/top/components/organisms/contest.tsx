import React from "react";
import styled from "styled-components";

import * as color from "app/components/color";
import { Contest } from "model/contest";

import { ReactComponent as FireIcon } from "../atoms/images/fire.svg";
import { ReactComponent as TimerBlackIcon } from "../atoms/images/clock-b.svg";

interface Props {
  contest: Contest;
}

export const ContestComponent: React.FC<Props> = ({ contest }) => {
  if (contest.status === "upcoming") {
    return (
      <Container>
        <ContestImage></ContestImage>
        <ContestDetail>
          <Category>{contest.category}</Category>
          <Title>{contest.title}</Title>
          <StartAt>
            <Timer />
            <Date>{contest.startAt}</Date>
          </StartAt>
        </ContestDetail>
      </Container>
    );
  } else if  (contest.status === "open") { 
    return (
      <Container>
        <ContestImage></ContestImage>
        <ContestDetail>
          <Category>{contest.category}</Category>
          <Title>{contest.title}</Title>
          <OnGoing>
            <Fire />
            <OnGoingText>開催中！</OnGoingText>
          </OnGoing>
        </ContestDetail>
      </Container>
    );
  } else {
    return null
  }
}

const Container = styled.div`
  width: 100%;
  margin-bottom: 26px;
  border-radius: 4px;
  box-shadow: 2px 2px 4px 0 ${color.BlackColor.rgba(0.5)};
`;

const ContestImage = styled.div`
  height: 132px;
  background: ${color.TextBaseColor.hex};
  border-radius: 4px 4px 0 0;
`;

const ContestDetail = styled.div`
  padding: 16px 16px 10px 16px;
  background: ${color.WhiteBaseColor.hex};
  border-radius: 0 0 4px 4px;
`;

const Category = styled.div`
  font-size: 16px;
  font-weight: 800;
  color: ${color.TextBaseColor.hex};
  margin-bottom: 12px;
`;

const Title = styled.div`
  font-size: 14px;
  font-weight: 600;
  color: ${color.TextBaseColor.hex};
  margin-bottom: 12px;
`;

const StartAt = styled.div`
  display: flex;
  justify-content: flex-start;
  align-items: center;
`;

const Timer = styled(TimerBlackIcon)`
  width: 14px;
  height: 14px;
  margin-right: 4px;
`;

const Date = styled.div`
  font-size: 12px;
  color: ${color.TextBaseColor.hex};
`;

const OnGoing = styled.div`
  display: flex;
  justify-content: flex-start;
  align-items: center;
  width: 71px;
  height: 20px;
  border-radius: 2px;
  background-color: ${color.MainRed.hex};
`;

const Fire = styled(FireIcon)`
  width: 14px;
  height: 14px;
  margin: 0 4px;
`;

const OnGoingText = styled.div`
  font-size: 12px;
  color: ${color.WhiteBaseColor.hex};
`;