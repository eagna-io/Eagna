import React from "react";
import ReactDOM from "react-dom";
import ReactGA from "react-ga";
import { createStore, applyMiddleware } from "redux";
import thunk from "redux-thunk";
import logger from "redux-logger";

import { rootReducer } from "app/redux";
import { checkLogin } from "app/redux/user";
import App from "app/app";

ReactGA.initialize("UA-147662091-1", {
  gaOptions: { siteSpeedSampleRate: 100 }
});

const store = createStore(rootReducer, applyMiddleware(thunk, logger));
store.dispatch<any>(checkLogin());

ReactDOM.render(<App store={store} />, document.getElementById("root"));
