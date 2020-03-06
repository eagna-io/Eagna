import React from "react";
import { useSelector } from "react-redux";
import styled from "styled-components";

import { RootState } from "app/redux";

import Chart from "../molecules/chart";


const ChartContainer: React.FC = () => {
  const datasets = useSelector((state: RootState) =>state.chart.datasets);

  return (
    <Container>
      <Chart height={300} datasets={datasets} />
    </Container>
  );
};

export default ChartContainer;

const Container = styled.div`
  z-index: -100;
  position: absolute;
  width: 100vw;
  top: 50%;
  left: 50%;
  transform: translateY(-50%) translateX(-50%);
  -webkit- transform: translateY(-50%) translateX(-50%);
  background-color: rgba(36, 36, 35);
`;