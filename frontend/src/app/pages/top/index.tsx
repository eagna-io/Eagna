import React from "react";
import { useDispatch, useSelector } from "react-redux";
import styled from "styled-components";

import { RootState } from "app/redux";
import { actions, Data } from "app/redux/chart";
import { now } from "model/time";
import Chart from "./components/chart";
import Feed from "./components/feed";
import Buttons from "./components/button";

const Page: React.FC = () => {
  const [datasets, records, userScore] = useSelector((state: RootState) => [
    state.chart.datasets,
    state.chart.records,
    state.chart.userScore
  ]);
  const dispatch = useDispatch();

  React.useEffect(() => {
    let handler = setInterval(() => {
      const user = botNames[Math.floor(Math.random() * 1000) % botNames.length];
      const time = now();
      const outcome = Math.random() >= 0.5 ? "win" : "lose";
      dispatch(actions.vote({ outcome, time, user }));
    }, 500);

    return () => {
      clearInterval(handler);
    };
  }, [dispatch]);

  const publicPred = getPublicPrediction(datasets.win);
  const userScoreStr = `${userScore}`.slice(0, 4);
  const themeTitle = 'RAGE Shadowverse 2020 Spring\nGRAND FINALS Shimon/REVが優勝するか？'

  return (
    <Container>
      <Title>
        <ThemeTitle style={{whiteSpace: 'pre-line'}}>
          {themeTitle}
        </ThemeTitle>
      </Title>
      <Contents>
        <ChartContainer>
          <MyScore>
            <PredictionTitle>あなたのスコア</PredictionTitle>
            <PredictionValue>{userScoreStr}</PredictionValue>
          </MyScore>
          <PublicPrediction>
            <PredictionTitle>みんなの予想した優勝確率</PredictionTitle>
            <PredictionValue>
              <AlignRight>
                {publicPred}
                <Small>%</Small>
              </AlignRight>
            </PredictionValue>
          </PublicPrediction>
          <Chart height={300} datasets={datasets} />
        </ChartContainer>
        <Feed records={records} />
        <Buttons
          onVote={outcome =>
            dispatch(
              actions.vote({ outcome, time: now(), user: "たかはしあつき" })
            )
          }
        />
      </Contents>
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

const botNames = [
  "ふるさわゆうや",
  "ふなはしこうき",
  "ドナルドトランプ",
  "きしべろはん"
];

const Container = styled.div`
  width: 100%;
  overflow: scroll;
  background-color: #242423;
`;

const Contents = styled.div`
  width: 100vw;
  height: 100vh;
  padding: 20px;
  background-color: #121212;
`;

const Title = styled.div`
  position: relative;
  background-color: #242423;
  padding: 20px;
`;

const ChartContainer = styled.div`
  position: relative;
  height: 380px;
  padding-top: 70px;
  margin-bottom: 15px;
  background-color: #242423;
`;

const MyScore = styled.div`
  position: absolute;
  width: fit-content;
  top: 10px;
  left: 10px;
`;

const PublicPrediction = styled.div`
  position: absolute;
  width: fit-content;
  top: 10px;
  right: 10px;
`;

const ThemeTitle = styled.div`
  color: white;
  font-size: 14px;
  font-weight: 500;
`;

const PredictionTitle = styled.div`
  color: #bb86fc;
  font-size: 10px;
  font-weight: 300;
`;

const PredictionValue = styled.div`
  color: white;
  font-size: 36px;
  font-weight: 300;
  font-family: "Montserrat";
`;

const AlignRight = styled.div`
  text-align: right;
`;

const Small = styled.span`
  font-size: 21px;
`;
