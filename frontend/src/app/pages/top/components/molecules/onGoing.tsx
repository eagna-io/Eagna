import React from "react";
import styled from "styled-components";

import * as color from "app/components/color";

import { ReactComponent as FireIcon } from "../atoms/images/fire.svg";

export const OnGoing: React.FC = () => {
  return (
    <Container>
      <Fire />
      <OnGoingText>開催中！</OnGoingText>
    </Container>
  )
}

const Container = styled.div`
  display: flex;
  justify-content: flex-start;
  align-items: center;
  width: 71px;
  height: 20px;
  border-radius: 2px;
  background-color: ${color.MainRed.hex};
`;

const Fire = styled(FireIcon)`
  width: 14px;
  height: 14px;
  margin: 0 4px;
`;

const OnGoingText = styled.div`
  font-size: 12px;
  color: ${color.WhiteBaseColor.hex};
`;
