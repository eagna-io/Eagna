import React from "react";
import { useDispatch, useSelector } from "react-redux";
import styled from "styled-components";
import { BackgroundMainColor, PurpleColor, TextBaseColor, RankingColor } from "app/components/color";

import { RootState } from "app/redux";
import { actions, Data } from "app/redux/chart";
import { now } from "model/time";

import Header from "./components/organisms/header";
import ChartContainer from "./components/organisms/chartContainer";
import Feed from "./components/organisms/feed";
import VoteButton from "./components/organisms/voteButton";

const Page: React.FC = () => {

  const [datasets, records] = useSelector((state: RootState) => [
    state.chart.datasets,
    state.chart.records
  ]);
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

  const publicPred = getPublicPrediction(datasets.win);

  return (
    <Container>
      <ChartContainer />
      <SubContainer>
        <Header userName="Yuya_F" />
        <MarketTitle>{marketTitle}</MarketTitle>
        <Ranking>
          予測ランキング
          <RankingValue>
            <RankNum>{ranking}</RankNum>位｜{paticipantsNum}人中
          </RankingValue>
        </Ranking>
        <Feed records={records} />
      </SubContainer>
      <Guide>
        <PredictionTheme>{predictionTheme}</PredictionTheme>
        <PublicPrediction>
          みんなの予想
          <PredictionValue>{publicPred}<Sm>%</Sm></PredictionValue>
        </PublicPrediction>
      </Guide>
      <VoteButton
        onVote={outcome =>
          dispatch(
            actions.vote({ outcome, time: now(), user: "たかはしあつき" })
          )
        }
      />
    </Container>
  );
};

export default Page;

const getPublicPrediction = (data: Data[]): string => {
  if (data.length === 0) {
    return "-";
  } else {
    return Math.floor(data[data.length - 1][1] / 10).toString();
  }
};

const marketTitle = 'RAGE Shadowverse 2020 Spring'
const ranking = 2
const paticipantsNum = 358
const predictionTheme = 'GRAND FINALS Shimon/REVが優勝する確率を予想せよ'

const botNames = [
  "ふるさわゆうや",
  "ふなはしこうき",
  "ドナルドトランプ",
  "きしべろはん"
];


const Container = styled.div`
  width: 100vw;
  height: 100vh;
  background-color: ${BackgroundMainColor.hex};
`;

const SubContainer = styled.div`
  position: relative;
  width: 100vw;
  background-color: ${BackgroundMainColor.hexWithOpacity(0.5)};
  padding: 20px;
`;

const MarketTitle = styled.div`
  font-size: 18px;
  color: ${PurpleColor.hex};
  font-weight: 300;
`;

const Ranking = styled.div`
  margin-top: 8px;
  color: ${TextBaseColor.hex};
  font-size: 14px;
  font-weight: 600;
`;

const RankingValue = styled.div`
  margin-left: 8px;
  letter-spacing: 1px;
`;

const RankNum = styled.span`
  font-size: 24px;
  color: ${RankingColor.hex};
  font-weight: 800;
  margin-right: 4px;
`;

const Guide = styled.div`
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0px 20px 0px 20px;
`;

const PredictionTheme = styled.div`
  color: ${TextBaseColor.hex}
  font-size: 14px;
  whiteSpace: 'pre-line';
  width: 70%;
`;

const PublicPrediction = styled.div`
  color: ${TextBaseColor.hex}
  text-align: center;
  font-size: 14px;
  font-weight: 600;
  width: 30%;
`;

const PredictionValue = styled.div`
  font-size: 56px;
  line-height: 56px;
  font-weight: 800;
`;

const Sm = styled.span`
  font-size: 20px;
  font-weight: 800;
`;

