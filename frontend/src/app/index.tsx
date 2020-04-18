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
        render={() => <CloseContest contests={contests}/>}
      />
      <Redirect to="/" />
    </Switch>
  </Router>
);

const contests = [
  {
    category: "NBA（バスケ）",
    title: "Los Angels Lakers vs Golden State Warriors",
    startAt: "2020.06.01｜11:00",
    status: "upcoming" as const
  },
  {
    category: "NBA（バスケ）",
    title: "Los Angels Lakers vs Golden State Warriors",
    startAt: "2020.06.01｜11:00",
    status: "open" as const
  },
  {
    category: "NBA（バスケ）",
    title: "Los Angels Lakers vs Golden State Warriors",
    startAt: "2020.06.01｜11:00",
    status: "closed" as const
  },
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
