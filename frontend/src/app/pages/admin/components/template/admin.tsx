import React from "react";
import styled from "styled-components";

import * as color from "app/components/color";

import { NavigationBar } from "../organisms/navbar";

export const AdminTemplate: React.FC = ({ children }) => {
  return (
    <Container>
      <NavBarComponent>
        <NavigationBar />
      </NavBarComponent>
      <Content>{children}</Content>
    </Container>
  );
};

const Container = styled.div`
  width: 100vw;
  height: 100vh;
  background-color: ${color.AdminBackgroundColor.hex};
  user-select: none;
  display: flex;
`;

const NavBarComponent = styled.div`
  width: 250px;
  height: 100vh;
  background-color: ${color.AdminMainColor.hex};
  padding-top: 30px;
`;

const Content = styled.div`
  width: 1142px;
  height: calc(100vh - 40px);
  margin: 20px 24px;
  background-color: ${color.WhiteBaseColor.hex};
  box-shadow: 0 1px 4px 0 ${color.BlackColor.rgba(0.5)};
  padding: 121px 282px 0 121px;
`;
