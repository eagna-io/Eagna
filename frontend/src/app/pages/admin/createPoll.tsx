import React from "react";
import styled from "styled-components";

import {
  AdminBackgroundColor,
  NavBarBackgroundColor,
  WhiteBaseColor,
  BlackColor
} from "app/components/color";

import { NavigationBar } from "./conponents/organisms/navbar";

export const CreatePoll: React.FC = () => {
  return (
    <Container>
      <NavBarComponent>
        <NavigationBar />
      </NavBarComponent>
      <Content>
        this is CREATE POLL
      </Content>
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

const NavBarComponent = styled.div`
  width: 250px;
  height: 100vh;
  background-color: ${NavBarBackgroundColor.hex};
  padding-top: 30px;
`;

const Content = styled.div`
  width: 1142px;
  height: calc(100vh - 40px);
  margin: 20px 24px;
  background-color: ${WhiteBaseColor.hex};
  box-shadow: 0 1px 4px 0 ${BlackColor.rgba(0.5)};
`;
