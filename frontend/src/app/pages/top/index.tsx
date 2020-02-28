import React from "react";
import { useDispatch } from "react-redux";
import styled from "styled-components";

import { actions } from "app/redux/chart";
import { DateTime, now } from "model/time";
import Chart from "./components/chart";

type Data = [DateTime, number];
type Series = { name: string; data: Data[] }[];

const Page: React.FC = () => {
  const dispatch = useDispatch();

  React.useEffect(() => {
    let handler = setInterval(() => {
      if (Math.random() > 0.5) {
        dispatch(actions.vote({ outcome: "win", time: now() }));
      } else {
        dispatch(actions.vote({ outcome: "lose", time: now() }));
      }
    }, 100);

    return () => {
      clearInterval(handler);
    };
  }, [dispatch]);

  return (
    <Background>
      <ChartContainer>
        <Chart height={300} />
      </ChartContainer>
    </Background>
  );
};

export default Page;

const Background = styled.div`
  width: 100vw;
  height: 100vh;
  padding: 20px;
  background-color: #121212;
`;

const ChartContainer = styled.div`
  height: 400px;
  padding-top: 90px;
  background-color: #242423;
`;
