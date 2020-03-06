import React from "react";
import { useDispatch, useSelector } from "react-redux";
import styled from "styled-components";

import { RootState } from "app/redux";
import { actions } from "app/redux/chart";
import { now } from "model/time";

import Feed from "../molecules/feed";

interface Props {
  marketTitle: string;
  ranking: number;
  paticipantsNum: number;
}

const Contents: React.FC<Props> = ({ marketTitle, ranking, paticipantsNum }) => {
  const records = useSelector((state: RootState) => state.chart.records);
  const dispatch = useDispatch();

  React.useEffect(() => {
    let handler = setInterval(() => {
      const user = botNames[Math.floor(Math.random() * 1000) % botNames.length];
      const time = now();
      const outcome = Math.random() >= 0.5 ? "win" : "lose";
      dispatch(actions.vote({ outcome, time, user }));
    }, 200);

    return () => {
      clearInterval(handler);
    };
  }, [dispatch]);

  return (
    <Container>
      <MarketTitle>{marketTitle}</MarketTitle>
      <Ranking>
        予測ランキング
        <RankingValue>
          <RankNum>{ranking}</RankNum>位｜{paticipantsNum}人中
        </RankingValue>
      </Ranking>
      <Feed records={records} />
    </Container>
  );
};

export default Contents;

const botNames = [
  "ふるさわゆうや",
  "ふなはしこうき",
  "ドナルドトランプ",
  "きしべろはん"
];


const Container = styled.div`
  position: relative;
  background-color: rgba(36, 36, 35);
  padding: 0 20px 20px;
`;

const MarketTitle = styled.div`
  font-size: 18px;
  color: #BB86FC;
  font-weight: 300;
`;

const Ranking = styled.div`
  margin-top: 8px;
  color: #AEAEAE;
  font-size: 14px;
  font-weight: 600;
`;

const RankingValue = styled.div`
  margin-left: 8px;
  letter-spacing: 1px;
`;

const RankNum = styled.span`
  font-size: 24px;
  color: #FAD160;
  font-weight: 800;
  margin-right: 4px;
`;
