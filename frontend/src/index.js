import React from 'react';
import ReactDOM from 'react-dom';

import App from './app';
import { pageFromPath } from 'src/router';

const initialAccessToken = localStorage.getItem('accessToken');
const initialPage = pageFromPath(window.location.pathname);

ReactDOM.render(
  <App
    initialAccessToken={initialAccessToken}
    initialPage={initialPage} />,
  document.getElementById('app')
)
