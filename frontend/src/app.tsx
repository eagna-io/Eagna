import React, {FC, useState, useEffect} from 'react';
import {BrowserRouter as Router, Route, Switch} from 'react-router-dom';
import * as firebase from 'firebase/app';
import 'firebase/auth';
import {createGlobalStyle} from 'styled-components';

import {Responsive} from 'components/responsive';
import TopPage from 'pages/top';
import LoginPage from 'pages/login';
import AccountPage from 'pages/account';
import MarketPage from 'pages/market';
import AdminAddMarketPage from 'pages/admin_add_market';
import AdminResolveMarketPage from 'pages/admin_resolve_market';
import PlainTextPage from 'pages/plain_text';
import NotFoundPage from 'pages/not_found';
import User from 'models/user';
import {getMe, createUser} from 'api/user';

const App: FC<{}> = () => {
  const [user, setUser] = useState<User | null>(null);

  useEffect(() => {
    // Firebase認証のステータスをwatch
    firebase.auth().onAuthStateChanged(fbUser => {
      if (fbUser == null) {
        setUser(null);
      } else {
        if (user === null) {
          // User取得プロセス
          fbUser
            .getIdToken()
            .then(token =>
              getMe(token).then(user => {
                if (user != null) {
                  return user;
                } else {
                  // Firebase認証は終わっているが、サーバーには登録されていない
                  if (fbUser.displayName == null || fbUser.email == null) {
                    // TODO
                    throw 'Cant get name or email from Firebase Auth';
                  } else {
                    return createUser({
                      accessToken: token,
                      name: fbUser.displayName,
                      email: fbUser.email,
                    });
                  }
                }
              }),
            )
            .then(user => {
              setUser(user);
            });
        }
      }
    });

    return () => {
      firebase.auth().onAuthStateChanged(() => null);
    };
  }, [setUser, user]);

  return (
    <>
      <GlobalStyle />
      <Responsive>
        <Router>
          <Switch>
            <Route
              path="/"
              exact
              render={({history}) => (
                <TopPage history={history} setUser={setUser} />
              )}
            />
            <Route
              path="/login"
              exact
              render={({history}) => (
                <LoginPage user={user} history={history} />
              )}
            />
            <Route
              path="/me"
              exact
              render={({history}) => (
                <AccountPage user={user} history={history} />
              )}
            />
            <Route
              path="/market/:id"
              render={({history, match}) => (
                <MarketPage
                  user={user}
                  history={history}
                  marketId={match.params.id}
                />
              )}
            />
            <Route
              path="/admin/add_market"
              exact
              render={() => <AdminAddMarketPage user={user} />}
            />
            <Route
              path="/admin/resolve_market"
              exact
              render={() => <AdminResolveMarketPage user={user} />}
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
      </Responsive>
    </>
  );
};

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
