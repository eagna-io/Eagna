import React from "react";
import styled from "styled-components";
import { BackgroundMainColor } from "app/components/color";

import Chart from "../molecules/chart";
import { Data } from "../../reducer";

interface Props {
  dataset: Data[];
}

export const ChartContainer: React.FC<Props> = ({ dataset }) => {
  return (
    <Container>
      <Chart height={"100%"} dataset={dataset} />
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
