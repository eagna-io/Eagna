import React from "react";
import styled from "styled-components";
import { Moment } from "moment";

import * as color from "app/components/color";

import { ReactComponent as TimerBlackIcon } from "../atoms/images/clock-b.svg";

interface Props {
  startAt?: Moment;
}

export const StartSchedule: React.FC<Props> = ({ startAt }) => {
  return (
    <Container>
      <TimerIcon />
      {startAt ? (
        <Date>{startAt.format("YYYY.MM.DD | HH:mm")} OPEN</Date>
      ) : (
        <Date>開催日未定</Date>
      )}
    </Container>
  );
};

const Container = styled.div`
  display: flex;
  justify-content: flex-start;
  align-items: center;
`;

const TimerIcon = styled(TimerBlackIcon)`
  width: 14px;
  height: 14px;
  margin-right: 4px;
`;

const Date = styled.div`
  font-size: 12px;
  color: ${color.TextBaseColor.hex};
`;
