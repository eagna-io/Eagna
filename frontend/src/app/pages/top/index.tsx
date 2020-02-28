import React from "react";
import { useDispatch, useSelector } from "react-redux";
import styled from "styled-components";

import { RootState } from "app/redux";
import { actions, Data } from "app/redux/chart";
import { DateTime, now } from "model/time";
import Chart from "./components/chart";

const Page: React.FC = () => {
  const datasets = useSelector((state: RootState) => state.chart.datasets);
  const dispatch = useDispatch();

  React.useEffect(() => {
    let handler = setInterval(() => {
      const user = botNames[Math.floor(Math.random() * 1000) % 3];
      const time = now();
      const outcome = Math.random() >= 0.5 ? "win" : "lose";
      dispatch(actions.vote({ outcome, time, user }));
    }, 100);

    return () => {
      clearInterval(handler);
    };
  }, [dispatch]);

  const publicPred = getPublicPrediction(datasets.win);

  return (
    <Background>
      <ChartContainer>
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
    </Background>
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

const botNames = ["たかはしあつき", "ふなはしこうき", "ドナルドトランプ"];

const Background = styled.div`
  width: 100vw;
  height: 100vh;
  padding: 20px;
  background-color: #121212;
`;

const ChartContainer = styled.div`
  position: relative;
  height: 400px;
  padding-top: 90px;
  margin-top: 120px;
  background-color: #242423;
`;

const PublicPrediction = styled.div`
  position: absolute;
  display: inline-block;
  width: fit-content;
  top: 10px;
  right: 10px;
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

const AlignLeft = styled.div`
  text-align: left;
`;

const Small = styled.span`
  font-size: 21px;
`;
