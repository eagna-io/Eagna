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
  display: flex;
  width: 100vw;
  height: 100vh;
  background-color: ${color.AdminBackgroundColor.hex};
  user-select: none;
`;

const NavBarComponent = styled.div`
  width: 250px;
  height: 100vh;
  padding-top: 30px;
  background-color: ${color.AdminMainColor.hex};  
`;

const Content = styled.div`
  position: relative;
  width: 1142px;
  height: calc(100vh - 40px);
  margin: 20px 24px;
  padding: 121px 282px 0 121px;
  background-color: ${color.WhiteBaseColor.hex};
  box-shadow: 0 1px 4px 0 ${color.BlackColor.rgba(0.5)};
`;
