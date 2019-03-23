import React from 'react';

export const AccessTokenContext = React.createContext({
  token: null,
  setToken: () => {},
});

export const RouterContext = React.createContext(null);
