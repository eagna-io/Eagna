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
import {getMe} from 'api/user';

const App: FC<{}> = () => {
  const [user, setUser] = useState<User | null>(null);

  useEffect(() => {
    firebase.auth().onAuthStateChanged(fbUser => {
      if (fbUser == null) {
        setUser(null);
      } else {
        fbUser
          .getIdToken()
          .then(token => getMe(token))
          .then(user => {
            if (user != null) {
              setUser(user);
            } else {
              // Firebase認証は終わっているが、サーバーには登録されていない
              console.log(fbUser);
            }
          });
      }
    });
  });

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
