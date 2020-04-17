import React from "react";
import styled from "styled-components";

import * as color from "app/components/color";
import { Contest } from "model/contest";

import { ReactComponent as FireIcon } from "../atoms/images/fire.svg";
import { ReactComponent as TimerBlackIcon } from "../atoms/images/clock-b.svg";

interface Props {
  contest: Contest;
}

export const Schedule: React.FC<Props> = ({ contest }) => {
  if (contest.status === "upcoming") {
    return (
      <StartAt>
        <Timer />
        <Date>{contest.startAt} OPEN</Date>
      </StartAt>
    );
  } else if (contest.status === "open") { 
    return (
      <OnGoing>
        <Fire />
        <OnGoingText>開催中！</OnGoingText>
      </OnGoing>
    );
  } else {
    return null
  }
}

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
