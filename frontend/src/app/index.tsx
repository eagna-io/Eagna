import React, { FC } from "react";
import { Provider as ReduxProvider } from "react-redux";
import {
  BrowserRouter as Router,
  Route,
  Redirect,
  Switch
} from "react-router-dom";
import { createGlobalStyle } from "styled-components";

import { store } from "./redux";
import { MarketPage } from "./pages/market";

const App: FC = () => {
  return (
    <>
      <GlobalStyle />
      <ReduxProvider store={store}>
        <AppRouter />
      </ReduxProvider>
    </>
  );
};

export default App;

const AppRouter: FC = () => (
  <Router>
    <Switch>
      <Route
        path="/market/:id"
        exact
        render={({ match }) => <MarketPage marketId={match.params.id} />}
      />
      <Redirect to="/market" />
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
