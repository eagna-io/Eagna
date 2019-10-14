import React, { FC } from "react";
import { BrowserRouter as Router, Route, Switch } from "react-router-dom";
import { createGlobalStyle } from "styled-components";
import { Provider as ReduxProvider } from "react-redux";
import { StylesProvider } from "@material-ui/core/styles";

import { Store } from "./redux";
import TopPage from "./pages/top";
import LoginPage from "./pages/login";
import AccountPage from "./pages/account";
import SpecialSignupPage from "./pages/special_signup";
import MarketPage from "./pages/market";
import PrizeListPage from "./pages/prize-list";
import AdminAddMarketPage from "./pages/admin_add_market";
import AdminResolveMarketPage from "./pages/admin_resolve_market";
import PlainTextPage from "./pages/plain_text";
import NotFoundPage from "./pages/not_found";

const App: FC<{ store: Store }> = ({ store }) => {
  return (
    <>
      <GlobalStyle />
      <StylesProvider injectFirst>
        <ReduxProvider store={store}>
          <AppRouter />
        </ReduxProvider>
      </StylesProvider>
    </>
  );
};

export default App;

const AppRouter: FC = () => (
  <Router>
    <Switch>
      <Route path="/" exact component={TopPage} />
      <Route path="/login" exact component={LoginPage} />
      <Route path="/account" exact component={AccountPage} />
      <Route
        path="/market/:id"
        exact
        render={({ match }) => <MarketPage marketId={match.params.id} />}
      />
      <Route path="/prize" exact component={PrizeListPage} />
      <Route path="/admin/add_market" exact component={AdminAddMarketPage} />
      <Route
        path="/admin/resolve_market"
        exact
        component={AdminResolveMarketPage}
      />
      <Route
        path="/privacy_policy"
        exact
        render={() => <PlainTextPage textUrl="/txt/privacy_policy.txt" />}
      />
      <Route
        path="/terms"
        exact
        render={() => <PlainTextPage textUrl="/txt/terms.txt" />}
      />
      <Route path="/special/signup" exact component={SpecialSignupPage} />
      <Route render={() => <NotFoundPage />} />
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
