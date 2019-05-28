import React from 'react';
import ReactDOM from 'react-dom';
import firebase from 'firebase';

import App from './app';

const firebaseApiKey = process.env.REACT_APP_FIREBASE_API_KEY;
const firebaseConfig = {
  apiKey: firebaseApiKey,
  authDomain: 'rohanmarket.firebaseapp.com',
  databaseURL: 'https://rohanmarket.firebaseio.com',
  projectId: 'rohanmarket',
  storageBucket: 'rohanmarket.appspot.com',
  messagingSenderId: '183815697689',
  appId: '1:183815697689:web:6b90aa7474274aa6',
};
firebase.initializeApp(firebaseConfig);

ReactDOM.render(<App />, document.getElementById('root'));