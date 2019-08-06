import React, {FC} from 'react';
import {BrowserRouter as Router, Route, Switch} from 'react-router-dom';
import {createGlobalStyle} from 'styled-components';

import {User} from 'models/user';
import {getMe, createUser} from 'api/user';
import {Responsive} from './components/responsive';
import {UserProvider} from './components/user';
import TopPage from './pages/top';
import LoginPage from './pages/login';
import AccountPage from './pages/account';
// import MarketPage from 'pages/market';
import SpecialSignupPage from './pages/special_signup';
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
      <Route path="/" exact render={() => <TopPage />} />
      <Route path="/login" exact render={() => <LoginPage />} />
      <Route path="/account" exact render={() => <AccountPage />} />
      <Route
        path="/admin/add_market"
        exact
        render={() => <AdminAddMarketPage />}
      />
      <Route
        path="/admin/resolve_market"
        exact
        render={() => <AdminResolveMarketPage />}
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
      color: #1B384E;
    }
    a: visited {
      color: #1B384E;
    }
    p {
      margin: 0;
      padding: 0;
    }
  `;
