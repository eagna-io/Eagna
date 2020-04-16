import React from "react";
import styled from "styled-components";

import * as color from "app/components/color";

export const Top: React.FC = () => {
  return (
    <Container>

    </Container>
  );
}

const Container = styled.div`
  position: relative;
  width: 100vw;
  height: 100vh;
  padding: 16px 28px;
  background-image: linear-gradient(151deg, ${color.WildWatermelon.hex} 0%, ${color.ToreaBay.hex} 100%);
  user-select: none;
`;
