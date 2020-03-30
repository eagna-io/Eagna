import React from "react";
import styled from "styled-components";

import {
  WhiteBaseColor,
  ShadowGray,
  Correct,
  MainRed
} from "app/components/color";
import { Timer as TimerModel } from "../../models";

interface Props {
  content: TimerModel;
}

export const Timer: React.FC<Props> = ({ content }) => {
  if (content === "closed") {
    return <ClosedTimer>CLOSED</ClosedTimer>;
  } else if (content === "correct") {
    return <CorrectTimer>正解！</CorrectTimer>;
  } else if (content === "incorrect") {
    return <IncorrectTimer>残念..</IncorrectTimer>;
  } else {
    return <CountDownTimer>{formatTime(content)}</CountDownTimer>;
  }
};

const formatTime = (seconds: number) => {
  const min = Math.floor(seconds / 60).toString();
  const sec = `0${seconds % 60}`.slice(-2);
  return `${min}:${sec}`;
};

const ClosedTimer = styled.div`
  width: 71px;
  height: 71px;
  margin 0px auto;
  border-radius: 50%;
  margin-bottom: 24px;
  text-align: center;
  line-height: 71px;
  font-size: 12px;
  font-weight: 800;
  color: ${WhiteBaseColor.hex};
  background-color: ${ShadowGray.hex};
`;

const CorrectTimer = styled.div`
  width: 71px;
  height: 71px;
  margin 0px auto;
  border-radius: 50%;
  border: solid 3px ${ShadowGray.hex};
  margin-bottom: 24px;
  text-align: center;
  line-height: 65px;
  font-size: 16px;
  font-weight: 800;
  color: ${WhiteBaseColor.hex};
  background-color: ${Correct.hex};
`;

const IncorrectTimer = styled.div`
  width: 71px;
  height: 71px;
  margin 0px auto;
  border-radius: 50%;
  border: solid 3px ${ShadowGray.hex};
  margin-bottom: 24px;
  text-align: center;
  line-height: 65px;
  font-size: 16px;
  font-weight: 800;
  color: ${WhiteBaseColor.hex};
  background-color: ${MainRed.hex};
`;

const CountDownTimer = styled.div`
  width: 71px;
  height: 71px;
  margin 0px auto;
  border-radius: 50%;
  border: solid 3px ${ShadowGray.hex};
  margin-bottom: 24px;
  text-align: center;
  line-height: 65px;
  font-size: 16px;
  font-weight: 800;
  color: ${WhiteBaseColor.hex};
`;
