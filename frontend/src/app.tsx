import React, {FC, useState, useEffect} from 'react';
import {BrowserRouter as Router, Route, Switch} from 'react-router-dom';
import firebase from 'firebase';
import {createGlobalStyle} from 'styled-components';

import TopPage from 'pages/top';
import LoginPage from 'pages/login';
import AccountPage from 'pages/account';
import MarketPage from 'pages/market';
import AdminPage from 'pages/admin';
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
    });
  }, []);

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

  return (
    <>
      <GlobalStyle />
      <Router>
        <Switch>
          <Route path="/" exact render={() => <TopPage />} />
          <Route
            path="/login"
            exact
            render={({history}) => <LoginPage user={user} history={history} />}
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
            path="/admin"
            exact
            render={() =>
              user && user.isAdmin ? (
                <AdminPage user={user} />
              ) : (
                <NotFoundPage />
              )
            }
          />
          <Route render={() => <NotFoundPage />} />
        </Switch>
      </Router>
    </>
  );
};

export default App;
