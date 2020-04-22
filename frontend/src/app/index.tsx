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
import { CreateContest } from "./pages/admin/createContest";
import { CloseContest } from "./pages/admin/closeContest";
import { OpenContest } from "./pages/admin/openContest";
import moment from "moment";

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
        path="/contest/:id"
        exact
        render={({ match }) => <InstapollPage contestId={ match.params.id } />}
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
        path="/admin/poll/create"
        exact
        render={() => <CreatePoll />}
      />
      <Route
        path="/admin/poll/resolve"
        exact
        render={() => <ResolvePoll />}
      />
      <Route
        path="/admin/contest/create"
        exact
        render={() => <CreateContest />}
      />
      <Route
        path="/admin/contest/close"
        exact
        render={() => <CloseContest contests={[]}/>}
      />
      <Route
        path="/admin/contest/open"
        exact
        render={() => <OpenContest contests={[]}/>}
      />
      <Redirect to="/" />
    </Switch>
  </Router>
);

const contests = [
  {
    id: "1",
    category: "NBA（バスケ）",
    title: "Los Angels Lakers vs Golden State Warriors",
    status: "Upcoming" as const
  },
  {
    id: "2",
    category: "NBA（バスケ）",
    title: "Los Angels Lakers vs Golden State Warriors",
    status: "Open" as const
  },
  {
    id: "3",
    category: "NBA（バスケ）",
    title: "Los Angels Lakers vs Golden State Warriors",
    status: "Closed" as const
  }
];

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
