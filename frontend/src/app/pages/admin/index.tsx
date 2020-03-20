import React from "react";
import styled from "styled-components";

import {
  AdminBackgroundColor,
  NavBarBackgroundColor,
  WhiteBaseColor,
  BlackColor
} from "app/components/color";

export const Admin: React.FC = () => {
  return (
    <Container>
      <NavBar></NavBar>
      <Content></Content>
    </Container>
  );
}

const Container = styled.div`
  width: 100vw;
  height: 100vh;
  background-color: ${AdminBackgroundColor.hex};
  user-select: none;
  display: flex;
`;

const NavBar = styled.div`
  width: 250px;
  height: 100vh;
  background-color: ${NavBarBackgroundColor.hex};
`;

const Content = styled.div`
  width: 1142px;
  height: calc(100vh - 40px);
  margin: 20px 24px;
  background-color: ${WhiteBaseColor.hex};
  box-shadow: 0 1px 4px 0 ${BlackColor.rgba(0.5)};
`;
