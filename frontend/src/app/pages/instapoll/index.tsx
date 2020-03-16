import React from "react";
import styled from "styled-components";

import { BackgroundMainColor } from "app/components/color";

export const InstapollPage: React.FC = () => {
  return (
    <Container>x</Container>
  );
}

const Container = styled.div`
  width: 100vw;
  height: calc(100vh - 75px);
  background-color: ${BackgroundMainColor.hex};
  user-select: none;
`;