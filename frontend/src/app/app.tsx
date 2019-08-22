import React, {FC} from 'react';
import {BrowserRouter as Router, Route, Switch} from 'react-router-dom';
import {createGlobalStyle} from 'styled-components';

import {Responsive} from './components/responsive';
import {UserProvider} from './components/user';
import TopPage from './pages/top';
import LoginPage from './pages/login';
import AccountPage from './pages/account';
import SpecialSignupPage from './pages/special_signup';
import MarketPage from './pages/market';
import AdminAddMarketPage from './pages/admin_add_market';
import AdminResolveMarketPage from './pages/admin_resolve_market';
import PlainTextPage from './pages/plain_text';
import NotFoundPage from './pages/not_found';

const App: FC<{}> = () => {
  return (
    <>
      <GlobalStyle />
      <UserProvider>
        <Responsive>
          <AppRouter />
        </Responsive>
      </UserProvider>
    </>
  );
};

const AppRouter: FC = () => (
  <Router>
    <Switch>
      <Route path="/" exact component={TopPage} />
      <Route path="/login" exact component={LoginPage} />
      <Route path="/account" exact component={AccountPage} />
      <Route
        path="/market/:id"
        exact
        render={({match}) => <MarketPage marketId={match.params.id} />}
      />
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

export default App;

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
