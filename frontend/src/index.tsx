import React from "react";
import ReactDOM from "react-dom";
import * as firebase from "firebase/app";
import "firebase/auth";
import ReactGA from "react-ga";
import { createStore, applyMiddleware } from "redux";
import thunk from "redux-thunk";
import logger from "redux-logger";

import { rootReducer } from "app/redux";
import { startObservingUserLogin } from "app/redux/user";
import App from "app/app";

const firebaseApiKey = process.env.REACT_APP_FIREBASE_API_KEY;
const firebaseConfig = {
  apiKey: firebaseApiKey,
  authDomain: "rohanmarket.firebaseapp.com",
  databaseURL: "https://rohanmarket.firebaseio.com",
  projectId: "rohanmarket",
  storageBucket: "rohanmarket.appspot.com",
  messagingSenderId: "183815697689",
  appId: "1:183815697689:web:6b90aa7474274aa6"
};
firebase.initializeApp(firebaseConfig);

ReactGA.initialize("UA-147662091-1", {
  gaOptions: { siteSpeedSampleRate: 100 }
});

const store = createStore(rootReducer, applyMiddleware(thunk, logger));
store.dispatch<any>(startObservingUserLogin());

ReactDOM.render(<App store={store} />, document.getElementById("root"));
