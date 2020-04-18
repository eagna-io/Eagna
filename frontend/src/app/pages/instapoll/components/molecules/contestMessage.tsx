import React from "react";
import styled from "styled-components";

import { TextBaseColor } from "app/components/color";
import { Contest } from "model/contest";

import { ReactComponent as TimerIcon } from "../atoms/images/timer.svg";

interface Props {
  contest: Contest;
  numer?: number;
  denom?: number;
}

export const ContestBoard: React.FC<Props> = ({ contest, numer, denom }) => {
  return (
    <Container>
      <StatusMessage noMargin={contest.status === "Open"}>
        {contest.status === "Upcoming" ? message[0] : ""}
        {contest.status === "Open" ? message[1] : ""}
        {contest.status === "Closed" ? message[2] : ""}
        {contest.status === "Archived" ? message[2] : ""}
      </StatusMessage>
      <Schedule>
        {contest.status === "Upcoming" ? (
          <>
            <Timer />
            <ScheduleDetail>{contest.event_start_at}</ScheduleDetail>
          </>
        ) : (
          ""
        )}
        {contest.status === "Closed" || contest.status === "Archived" ? (
          <ScoreBoard>
            あなたのスコア｜<Score>{numer}</Score>
            <Slash>/</Slash>
            <Score>{denom}</Score>問正解!!
          </ScoreBoard>
        ) : (
          ""
        )}
      </Schedule>
    </Container>
  );
};

const message = [
  "試合開始までお待ち下さい。",
  "予想クイズの作成中です。\nしばらくお待ち下さい。",
  "コンテストは終了しました。"
];

const Container = styled.div`
  width: 100%;
  color: ${TextBaseColor.hex};
`;

const StatusMessage = styled.div<{ noMargin: boolean }>`
  margin-bottom: ${props => (props.noMargin ? "0" : "10px")};
  font-size: 16px;
  font-weight: 800;
  letter-spacing: 1.56px;
  white-space: pre-wrap;
`;

const Schedule = styled.div`
  display: flex;
  justify-content: flex-start;
  align-items: center;
`;

const ScheduleDetail = styled.div`
  font-size: 12px;
  letter-spacing: 1.17px;
`;

const Timer = styled(TimerIcon)`
  width: 12px;
  height: 12px;
  margin-right: 8px;
  line-height: 12px;
`;

const ScoreBoard = styled.div`
  font-size: 12px;
  letter-spacing: 1.17px;
`;

const Score = styled.span`
  margin: 0 4px;
  font-size: 20px;
  font-weight: 800;
`;

const Slash = styled.span`
  vertical-align: 2px;
  font-size: 10px;
  font-weight: 600;
`;
