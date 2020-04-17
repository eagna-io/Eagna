import React, { FC } from "react";
import {
  BrowserRouter as Router,
  Route,
  Redirect,
  Switch
} from "react-router-dom";
import { createGlobalStyle } from "styled-components";

import { Top } from "./pages/top";
import { InstapollPage } from "./pages/instapoll";
import { Admin } from "./pages/admin";
import { CreatePoll } from "./pages/admin/createPoll";
import { ResolvePoll } from "./pages/admin/resolvePoll";
import { AdminLogin } from "./pages/admin/login";

const App: FC = () => {
  return (
    <>
      <GlobalStyle />
      <AppRouter />
    </>
  );
};

export default App;

const AppRouter: FC = () => (
  <Router>
    <Switch>
      <Route
        path="/"
        exact
        render={() => <Top />}
      />
      <Route
        path="/instapoll"
        exact
        render={() => <InstapollPage />}
      />
      <Route
        path="/admin"
        exact
        render={() => <Admin />}
      />
      <Route
        path="/admin/login"
        exact
        render={() => <AdminLogin />}
      />
      <Route
        path="/admin/create"
        exact
        render={() => <CreatePoll />}
      />
      <Route
        path="/admin/resolve"
        exact
        render={() => <ResolvePoll />}
      />
      <Redirect to="/instapoll" />
    </Switch>
  </Router>
);

const GlobalStyle = createGlobalStyle`
  body {
    margin: 0;
    font-family: 'Noto Sans JP', sans-serif;
    color: #1B384E;
    letter-spacing: 0;
  }
  * {
    box-sizing: border-box;
  }
  a {
    text-decoration: none;
    outline: none;
    color: inherit;
  }
  a: visited {
    color: inherit;
  }
  p {
    margin: 0;
    padding: 0;
  }
  button {
    background-color: transparent;
    border: none;
    cursor: pointer;
    outline: none;
    padding: 0;
    appearance: none;
  }
`;
