import React from "react";
import { useSelector } from "react-redux";
import styled from "styled-components";
import { BackgroundMainColor } from "app/components/color";

import { RootState } from "app/redux";

import Chart from "../molecules/chart";


const ChartContainer: React.FC = () => {
  const datasets = useSelector((state: RootState) =>state.chart.datasets);

  return (
    <Container>
      <Chart height={"100%"} datasets={datasets} />
    </Container>
  );
};

export default ChartContainer;

const Container = styled.div`
  position: absolute;
  width: 100vw;
  margin-top: 20vh;
  height: 40vh;
  padding-left: 20px;
  background-color: ${BackgroundMainColor.hexWithOpacity(0.5)};
`;