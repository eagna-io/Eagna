import React, { FC } from "react";
import {
  BrowserRouter as Router,
  Route,
  Redirect,
  Switch
} from "react-router-dom";
import styled from "styled-components";

import {
  AdminBackgroundColor,
  NavBarBackgroundColor,
  WhiteBaseColor,
  BlackColor
} from "app/components/color";

import { NavigationBar } from "./conponents/organisms/navbar";
import { CreatePoll } from "./conponents/organisms/createPoll";
import { ResolvePoll } from "./conponents/organisms/resolvePoll";

export const Admin: React.FC = () => {
  return (
    <Container>
      <NavBarComponent>
        <NavigationBar />
      </NavBarComponent>
      <Content>
        <AppRouter />
      </Content>
    </Container>
  );
}

const AppRouter: FC = () => (
  <Router>
    <Switch>
      <Route
        path="/admin/create"
        exact
        component={ CreatePoll }
      />
      <Route
        path="/admin/resolve"
        exact
        component={ResolvePoll}
      />
      <Redirect to="/admin/create" />
    </Switch>
  </Router>
);

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
