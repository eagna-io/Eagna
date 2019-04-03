import React from 'react';
import ReactDOM from 'react-dom';

import App from './app';

const initialAccessToken = localStorage.getItem('accessToken');

ReactDOM.render(
  <App
    initialAccessToken={initialAccessToken}
  />,
  document.getElementById('app')
)
