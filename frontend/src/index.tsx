import React from "react";
import ReactDOM from "react-dom";
import * as firebase from "firebase/app";
import "firebase/auth";
import ReactGA from "react-ga";

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

ReactDOM.render(<App />, document.getElementById("root"));
