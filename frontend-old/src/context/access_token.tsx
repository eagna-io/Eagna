import React from 'react';

export const AccessTokenContext = React.createContext({
  accessToken: null,
  setAccessToken: () => {},
});

export const RouterContext = React.createContext(null);
