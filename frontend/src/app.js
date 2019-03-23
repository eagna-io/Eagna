import React, { useState, useEffect } from 'react';
import styled from 'styled-components';

import LoginPage from 'src/pages/login';
import AccountPage from 'src/pages/account';
import MarketPage from 'src/pages/market';
import { AccessTokenContext, RouterContext } from 'src/context';
import Router, { Login, Account, Market, pageFromPath } from 'src/router';

export default function App(props) {
  const initialAccessToken = props.initialAccessToken;
  const initialPage = props.initialPage;
  const [token, setToken] = useState(initialAccessToken);
  const [page, setPage] = useState(initialPage);

  useEffect(() => {
    window.onpopstate = (event) => {
      setPage(pageFromPath(window.location.pathname))
    };
  });

  useEffect(() => {
    if (token) {
      localStorage.setItem('accessToken', token);
    } else {
      localStorage.removeItem('accessToken');
    }
  }, [token]);

  const router = new Router(setPage);
  const tokenCtx = {token, setToken}

  const renderPage = page => {
    console.log(page);
    switch(page.name) {
      case Login:
        return <LoginPage />
      case Account:
        return <AccountPage />
      case Market:
        return <MarketPage id={page.params.id} />
    }
  };

  return (
    <AccessTokenContext.Provider value={tokenCtx}>
      <RouterContext.Provider value={router}>
        { renderPage(page) }
      </RouterContext.Provider>
    </AccessTokenContext.Provider>
  )
}
