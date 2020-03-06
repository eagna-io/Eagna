import React from "react";
import { useSelector } from "react-redux";
import styled from "styled-components";
import { Color, BackgroundMainColor} from "app/components/color";

import { RootState } from "app/redux";

import Chart from "../molecules/chart";


const ChartContainer: React.FC = () => {
  const datasets = useSelector((state: RootState) =>state.chart.datasets);

  return (
    <Container bgcolor={BackgroundMainColor}>
      <Chart height={400} datasets={datasets} />
    </Container>
  );
};

export default ChartContainer;

const Container = styled("div")<{ bgcolor: Color }>`
  position: absolute;
  width: 100vw;
  background-color:  ${props => props.bgcolor.hex};
  margin-top: 30px;
`;