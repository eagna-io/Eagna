import React from "react";
import { useSelector } from "react-redux";
import styled from "styled-components";
import { BackgroundMainColor} from "app/components/color";

import { RootState } from "app/redux";

import Chart from "../molecules/chart";


const ChartContainer: React.FC = () => {
  const datasets = useSelector((state: RootState) =>state.chart.datasets);

  return (
    <Container>
      <Chart height={350} datasets={datasets} />
    </Container>
  );
};

export default ChartContainer;

const Container = styled.div`
  position: absolute;
  width: 100vw;
  margin-top: 30px;
`;