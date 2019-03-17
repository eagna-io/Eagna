import React, { useState, useEffect } from 'react';
import {
  BrowserRouter as Router,
  Route,
  Switch,
  Redirect
} from "react-router-dom";
import styled from 'styled-components';

import LoginPage from 'src/pages/login';
import AccountPage from 'src/pages/account';
import MarketPage from 'src/pages/market';
import { AccessTokenContext } from 'src/context';

export default function App() {
  const [token, setToken] = useState(null);
  const [isTokenInitialized, setIsTokenInitialized] = useState(false);
  const tokenCtx = {token, setToken}

  useEffect(() => {
    if (!isTokenInitialized) {
      setIsTokenInitialized(true);
      setToken(localStorage.getItem('accessToken'));
    } else {
      localStorage.setItem('accessToken', token);
    }
  }, [token]);

  return (
    <AccessTokenContext.Provider value={tokenCtx}>
      <Router>
        <Switch>
          <Route path="/login" component={LoginPage} />
          <Route path="/me" component={AccountPage} />
          <Route path="/market/:id" component={MarketPage} />
        </Switch>
      </Router>
    </AccessTokenContext.Provider>
  )
}
