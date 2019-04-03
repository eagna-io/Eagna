import React, { useState, useEffect } from 'react';
import { BrowserRouter as Router, Route, Switch, Redirect } from 'react-router-dom';

import LoginPage from 'src/pages/login';
import AccountPage from 'src/pages/account';
import MarketPage from 'src/pages/market';
import { AccessTokenContext } from 'src/context';

export default function App(props) {
  const initialAccessToken = props.initialAccessToken;
  const [token, setToken] = useState(initialAccessToken);

  useEffect(() => {
    if (token) {
      localStorage.setItem('accessToken', token);
    } else {
      localStorage.removeItem('accessToken');
    }
  }, [token]);

  return (
    <AccessTokenContext.Provider value={{token, setToken}}>
      <Router>
        <Switch>
          <Route path="/login" exact component={LoginPage} />
          <Route path="/me" exact component={AccountPage} />
          <Route path="/market/:id" component={MarketPage} />
          <Route render={() => <Redirect to="/login" />} />
        </Switch>
      </Router>
    </AccessTokenContext.Provider>
  )
}
