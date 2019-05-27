import React, { useState, useEffect } from 'react';
import { BrowserRouter as Router, Route, Switch, Redirect } from 'react-router-dom';

import LoginPage from 'pages/login';
import AccountPage from 'pages/account';
import MarketPage from 'pages/market';
import { AccessTokenContext } from 'context/access_token';

export default function App(props) {
  const initialAccessToken = props.initialAccessToken;
  const [accessToken, setAccessToken] = useState(initialAccessToken);

  useEffect(() => {
    if (accessToken) {
      localStorage.setItem('accessToken', accessToken);
    } else {
      localStorage.removeItem('accessToken');
    }
  }, [accessToken]);

  return (
    <AccessTokenContext.Provider value={{accessToken, setAccessToken}}>
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
