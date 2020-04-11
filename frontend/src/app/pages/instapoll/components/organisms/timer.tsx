import React from "react";
import styled from "styled-components";

import {
  WhiteBaseColor,
  Alto
} from "app/components/color";
import { Timer as TimerModel } from "model/poll";

interface Props {
  content: TimerModel;
}

export const Timer: React.FC<Props> = ({ content }) => {
  // TODO: state upcoming
  if (
    content === "closed"
  ) {
    return <Container>投票時間<TimeOut>締切</TimeOut></Container>;
  } else {
    return <Container>投票時間<CountDownTimer>{formatTime(content)}</CountDownTimer></Container>;
  }
};

const formatTime = (seconds: number) => {
  const min = Math.floor(seconds / 60).toString();
  const sec = `0${seconds % 60}`.slice(-2);
  return `${min}:${sec}`;
};

const Container = styled.div`
  width: 56px;
  margin-right: 21px;
  font-size: 8px;
  text-align: center;
  color: ${WhiteBaseColor.hex};
`;

const TimeOut = styled.div`
  font-size: 20px;
  letter-spacing: 0.71px;
  font-weight: 800;
  color: ${Alto.hex};
`;

const CountDownTimer = styled.div`
  font-size: 20px;
  letter-spacing: 0.71px;
  font-weight: 800;
  color: ${WhiteBaseColor.hex};
`;
