import React, {FC, useState, useEffect} from 'react';
import {
  BrowserRouter as Router,
  Route,
  Switch,
  Redirect,
} from 'react-router-dom';
import firebase from 'firebase';

import LoginPage from 'pages/login';
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
                  throw 'Cant get name or email from FB login';
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

  return (
    <Router>
      <Switch>
        <Route
          path="/login"
          exact
          render={({history}) => <LoginPage user={user} history={history} />}
        />
        <Route render={() => <Redirect to="/login" />} />
      </Switch>
    </Router>
  );
};

export default App;
